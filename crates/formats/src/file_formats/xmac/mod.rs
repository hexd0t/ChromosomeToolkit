//! Resources on the XMAC format:
//!  - O3DE (formerly Amazon Lumberyard) has an importer for XMAC, although a newer version
//!    at <https://github.com/o3de/o3de/tree/development/Gems/EMotionFX/Code/EMotionFX/Source>
//!    older commits contain more 'legacy' EMotionFX code
//!    (since Amazon bought EmotionFX & employs their devs, this can be considered 'ground truth')
//!    License: Apache 2 or MIT
//!  - Lumberyard's archived repo has more old pieces of EMotionFX left:
//!    at <https://github.com/aws/lumberyard/blob/master/dev/Gems/EMotionFX/Code/EMotionFX/Source/Importer/ActorFileFormat.h>
//!    License: AWS Agreement
//!  - RisenEditor:
//!    at <https://github.com/hhergeth/RisenEditor>
//!    License: none
//!  - Baltram's rmTools:
//!    at <https://github.com/Baltram/rmtools/blob/master/rmStuff/rmXmacReader.cpp>
//!    License: GPLv3

pub mod chunks;

use std::{io::Write, time::SystemTime};

use chunks::{
    XmacChunk,
    material::XmacStdMaterial,
    material_info::XmacMaterialInfo,
    mesh::XmacMesh,
    morph_targets::XmacMorphTargets,
    nodes::{XmacNodeId, XmacNodes},
    skinning_info::XmacSkinningInfo,
};
use serde::{Deserialize, Serialize};

use crate::{
    archive::{ArchiveReadTarget, ArchiveWriteTarget, TempWriteTarget},
    error::*,
    helpers::*,
    resourcefile::ResourceFile,
    types::time::DateTime,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacFile {
    pub res: ResourceFile,
    pub multiply_order: bool,
    pub chunks: Vec<chunks::XmacChunk>,
}

const R1_REV: [u8; 4] = *b"MA02";
const R1_CLASS: &str = "eCMotionActorResource2";
const R1_RAW_EXT: [u8; 8] = *b".xac\0\0\0\0";
const XMAC_MAGIC: [u8; 4] = *b"XAC ";

impl XmacFile {
    pub fn new(timestamp: SystemTime) -> Self {
        Self {
            res: ResourceFile {
                timestamp: DateTime::new(timestamp),
                props: Vec::new(),
                data_revision: R1_REV,
                class_name: R1_CLASS.to_string(),
                raw_file_ext: R1_RAW_EXT,
            },
            multiply_order: false,
            chunks: Vec::new(),
        }
    }

    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let res = ResourceFile::load(src)?;

        assert_eq!(&res.data_revision, &R1_REV);
        assert_eq!(&res.raw_file_ext[0..5], &R1_RAW_EXT[0..5]); //the last 3 bytes sometimes differ
        assert_eq!(&res.class_name, &R1_CLASS);

        let (chunks, multiply_order) = Self::load_xmac(src)?;

        let trail = read_u64(src)?;
        if trail != 0 {
            println!("Note: Trail is nonzero");
        }

        Ok(XmacFile {
            res,
            chunks,
            multiply_order,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        assert_eq!(self.res.props.len(), 1);
        assert_eq!(&self.res.props[0].name, "Boundary");

        let mut data = TempWriteTarget::new(dst);
        self.save_xmac(&mut data)?;
        let data = data.finish();

        // If multiply_order is set, the tail must NOT be part of the data,
        // else it needs to be counted (but both types have the tail in the file)
        let data_len = if self.multiply_order {
            data.len()
        } else {
            data.len() + 8
        };

        self.res.save(dst, data_len)?;
        dst.write_all(&data)?;
        write_u64(dst, 0)?;

        Ok(())
    }

    fn load_xmac<R: ArchiveReadTarget>(src: &mut R) -> Result<(Vec<chunks::XmacChunk>, bool)> {
        let data_len = read_u32(src)? as u64;
        let xmac_start = src.stream_position()?;
        let xmac_finish = xmac_start + data_len;

        let mut magic = vec![0; 4];
        src.read_exact(&mut magic)?;
        assert_eq!(&magic, &XMAC_MAGIC);

        let _actor_version_maj = read_u8(src)?;
        let _actor_version_min = read_u8(src)?;
        // The version check in R1 is broken -
        // it is supposed to only accept 3.0, but accepts anything
        // with either maj = 3 OR min = 0 -
        // and the shipped files have V1.0 oO

        let big_endian = read_bool(src)?;
        let multiply_order = read_bool(src)?;

        let mut chunks = Vec::new();
        while src.stream_position()? < xmac_finish {
            let new_chunk = chunks::XmacChunk::load(src, big_endian, multiply_order, &chunks)?;
            chunks.push(new_chunk);
        }

        Ok((chunks, multiply_order))
    }

    pub fn sanity_check(&self) -> Result<()> {
        self.sanity_check_chunk_counts()
        // ToDo:
        // - Check joints exist
        // - Check Skins reference existing Meshes
        // - Check Skin.for_collision_mesh == Mesh.collision_mesh
        // - Skin.table_entries.len == Mesh.orig_vertex_count
    }
    fn sanity_check_chunk_counts(&self) -> Result<()> {
        let mut info_chunks = 0;
        let mut nodes_chunks = 0;
        let mut mat_info_chunks = 0;
        let mut std_mat_chunks = 0;
        let mut mesh_chunks = 0;
        let mut skin_chunks = 0;
        let mut morph_chunks = 0;
        for chunk in &self.chunks {
            match chunk {
                XmacChunk::Info(_) => info_chunks += 1,
                XmacChunk::Nodes(_) => nodes_chunks += 1,
                XmacChunk::MaterialInfo(_) => mat_info_chunks += 1,
                XmacChunk::StdMaterial(_) => std_mat_chunks += 1,
                XmacChunk::Mesh(_) => mesh_chunks += 1,
                XmacChunk::SkinningInfo(_) => skin_chunks += 1,
                XmacChunk::MorphTargets(_) => morph_chunks += 1,
                XmacChunk::Unknown(_) => {}
            }
        }
        if info_chunks != 1 {
            println!("Warn: Expected 1 Info chunk, found {info_chunks}");
        }
        if nodes_chunks != 1 {
            return Err(Error::InvalidStructure(format!(
                "Expected 1 Nodes chunk, found {nodes_chunks}"
            )));
        }
        if mat_info_chunks != 1 {
            return Err(Error::InvalidStructure(format!(
                "Expected 1 Material Info chunk, found {mat_info_chunks}"
            )));
        }
        let mat_info = self.get_material_info_chunk().unwrap();
        if std_mat_chunks != mat_info.std_materials {
            return Err(Error::InvalidStructure(format!(
                "Expected {} Standard Material chunk, found {std_mat_chunks}",
                mat_info.std_materials
            )));
        }
        if mesh_chunks == 0 {
            return Err(Error::InvalidStructure(format!(
                "Expected at least 1 Mesh chunk, found {mesh_chunks}"
            )));
        }
        if skin_chunks > mesh_chunks {
            return Err(Error::InvalidStructure(format!(
                "Expected at most {mesh_chunks} Skin chunks, found {skin_chunks}"
            )));
        }
        if skin_chunks == 0 {
            println!(
                "Warn: Expected at least 1 Skin chunk, found none - the mesh will probably not be rendered!"
            );
        }
        if morph_chunks > mesh_chunks {
            return Err(Error::InvalidStructure(format!(
                "Expected at most {mesh_chunks} Skin chunks, found {morph_chunks}"
            )));
        }
        Ok(())
    }

    pub fn save_xmac<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        //self.sanity_check()?;

        let mut data = TempWriteTarget::new(dst);
        data.write_all(&XMAC_MAGIC)?;
        // Version Maj:
        write_u8(&mut data, 1)?;
        // Version Min:
        write_u8(&mut data, 0)?;

        let big_endian = false; // Big Endian doesn't make sense on x86, but its here if you need it
        let multiply_order = self.multiply_order; // Don't really know the influence

        write_bool(&mut data, big_endian)?;
        write_bool(&mut data, multiply_order)?;

        for chunk in &self.chunks {
            chunk.save(&mut data, big_endian)?;
        }
        let data = data.finish();

        write_u32(dst, data.len() as u32)?; //this one is always little-endian
        dst.write_all(&data)?;

        Ok(())
    }

    pub fn get_nodes_chunk(&self) -> Option<&XmacNodes> {
        fn get_nodes_chunk(chunk: &XmacChunk) -> Option<&XmacNodes> {
            if let XmacChunk::Nodes(nodes) = chunk {
                Some(nodes)
            } else {
                None
            }
        }
        self.chunks.iter().find_map(get_nodes_chunk)
    }

    pub fn get_mesh_chunks(&self) -> Vec<&XmacMesh> {
        fn get_mesh_chunk(chunk: &XmacChunk) -> Option<&XmacMesh> {
            if let XmacChunk::Mesh(mesh) = chunk {
                Some(mesh)
            } else {
                None
            }
        }
        self.chunks.iter().filter_map(get_mesh_chunk).collect()
    }

    pub fn get_mesh_chunk<'a>(&'a self, node_id: XmacNodeId) -> Option<&'a XmacMesh> {
        let get_mesh_chunk = |chunk: &'a XmacChunk| -> Option<&'a XmacMesh> {
            if let XmacChunk::Mesh(mesh) = chunk {
                if mesh.node_id == node_id {
                    Some(mesh)
                } else {
                    None
                }
            } else {
                None
            }
        };
        self.chunks.iter().find_map(get_mesh_chunk)
    }

    pub fn get_material_info_chunk(&self) -> Option<&XmacMaterialInfo> {
        fn get_material_info_chunk(chunk: &XmacChunk) -> Option<&XmacMaterialInfo> {
            if let XmacChunk::MaterialInfo(matinf) = chunk {
                Some(matinf)
            } else {
                None
            }
        }
        self.chunks.iter().find_map(get_material_info_chunk)
    }

    pub fn get_material_chunks(&self) -> Vec<&XmacStdMaterial> {
        fn get_material_chunk(chunk: &XmacChunk) -> Option<&XmacStdMaterial> {
            if let XmacChunk::StdMaterial(mat) = chunk {
                Some(mat)
            } else {
                None
            }
        }
        self.chunks.iter().filter_map(get_material_chunk).collect()
    }

    pub fn get_skinning_chunks(&self) -> Vec<&XmacSkinningInfo> {
        fn get_skinning_chunk(chunk: &XmacChunk) -> Option<&XmacSkinningInfo> {
            if let XmacChunk::SkinningInfo(skin) = chunk {
                Some(skin)
            } else {
                None
            }
        }
        self.chunks.iter().filter_map(get_skinning_chunk).collect()
    }

    pub fn get_morph_chunk(&self) -> Option<&XmacMorphTargets> {
        fn get_morph_chunk(chunk: &XmacChunk) -> Option<&XmacMorphTargets> {
            if let XmacChunk::MorphTargets(mat) = chunk {
                Some(mat)
            } else {
                None
            }
        }
        self.chunks.iter().find_map(get_morph_chunk)
    }
}

/// XMAC Strings store their length in (endianness-affected) u32
fn read_xmac_str<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<String> {
    let len = read_u32_endian(src, big_endian)?;
    if len > 255 {
        return Err(Error::InvalidStructure(format!(
            "String @{:x} is supposedly {len} bytes",
            src.stream_position()?
        )));
    }
    let len = len as usize;
    let mut str_buf = vec![0; len];
    src.read_exact(&mut str_buf)?;

    // TODO: Check if Xmac files contain UTF8?
    if let Some(string) =
        encoding_rs::WINDOWS_1252.decode_without_bom_handling_and_without_replacement(&str_buf)
    {
        Ok(string.to_string())
    } else {
        Err(Error::InvalidString(format!("{:x?}", str_buf)))
    }
}

fn write_xmac_str<W: ArchiveWriteTarget>(
    dst: &mut W,
    content: &str,
    big_endian: bool,
) -> Result<usize> {
    let (str_buf, _, unmappable) = encoding_rs::WINDOWS_1252.encode(content);
    write_u32_endian(dst, str_buf.len() as u32, big_endian)?;
    dst.write_all(&str_buf)?;
    if unmappable {
        println!("Warning: String '{content}' contains unmappable characters!");
    }
    Ok(str_buf.len() + 4)
}
