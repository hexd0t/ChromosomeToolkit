use std::collections::HashMap;

use mesh::XmacMesh;
use nodes::XmacNodeId;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::archive::TempWriteTarget;
use crate::error::*;
use crate::helpers::*;

pub mod info;
pub mod material;
pub mod material_info;
pub mod mesh;
pub mod morph_targets;
pub mod nodes;
pub mod skinning_info;
pub mod unknown;

#[derive(Debug, Deserialize, Serialize)]
pub enum XmacChunk {
    Info(info::XmacInfo),
    Nodes(nodes::XmacNodes),
    MaterialInfo(material_info::XmacMaterialInfo),
    StdMaterial(material::XmacStdMaterial),
    Mesh(mesh::XmacMesh),
    SkinningInfo(skinning_info::XmacSkinningInfo),
    MorphTargets(morph_targets::XmacMorphTargets),
    Unknown(unknown::XmacUnknownChunk),
}

pub struct XmacChunkMeta {
    pub type_id: u32,
    pub size: u32,
    pub version: u32,
}

#[repr(u32)]
#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
pub enum XmacChunkType {
    Node = 0,
    Mesh = 1,
    SkinningInfo = 2,
    /// R1's data has invalid chunk sizes for these for some reason...
    StdMaterial = 3,
    StdMaterialLayer = 4,
    FxMaterial = 5,
    Limit = 6,
    Info = 7,
    MeshLodLevels = 8,
    StdProgMorphTarget = 9,
    NodeGroups = 10,
    Nodes = 11,
    StdPMorphTargets = 12,
    MaterialInfo = 13,
    NodeMotionSources = 14,
    AttachmentNodes = 15,
    MaterialAttributeSet = 16,
    GenericMaterial = 17,
    PhysicsSetup = 18,
    SimulatedObjectSetup = 19,
    MeshAsset = 20,
}

impl XmacChunk {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        multiply_order: bool,
        prev_chunks: &[XmacChunk],
    ) -> Result<Self> {
        let chunk_meta = XmacChunkMeta::load(src, big_endian)?;
        let chunk_start = src.stream_position()?;
        let chunk_end = chunk_start + chunk_meta.size as u64;
        let result = if let Ok(chunk_type) = chunk_meta.type_id.try_into() {
            match chunk_type {
                XmacChunkType::Info => Self::load_unknown(
                    info::XmacInfo::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Info),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::Nodes => Self::load_unknown(
                    nodes::XmacNodes::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Nodes),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::MaterialInfo => Self::load_unknown(
                    material_info::XmacMaterialInfo::load(
                        src,
                        big_endian,
                        multiply_order,
                        &chunk_meta,
                    )?
                    .map(XmacChunk::MaterialInfo),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::StdMaterial => Self::load_unknown(
                    material::XmacStdMaterial::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::StdMaterial),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::Mesh => Self::load_unknown(
                    mesh::XmacMesh::load(src, big_endian, multiply_order, &chunk_meta)?
                        .map(XmacChunk::Mesh),
                    src,
                    &chunk_meta,
                ),
                XmacChunkType::SkinningInfo => {
                    let meshes = Self::get_meshes(prev_chunks);
                    Self::load_unknown(
                        skinning_info::XmacSkinningInfo::load(
                            src,
                            big_endian,
                            multiply_order,
                            &chunk_meta,
                            &meshes,
                        )?
                        .map(XmacChunk::SkinningInfo),
                        src,
                        &chunk_meta,
                    )
                }
                XmacChunkType::StdPMorphTargets => Self::load_unknown(
                    morph_targets::XmacMorphTargets::load(
                        src,
                        big_endian,
                        multiply_order,
                        &chunk_meta,
                    )?
                    .map(XmacChunk::MorphTargets),
                    src,
                    &chunk_meta,
                ),
                _ => {
                    println!(
                        "Unimplemented XMAC chunk {chunk_type:?}.{}@{:x}",
                        chunk_meta.version,
                        src.stream_position()?
                    );
                    XmacChunk::Unknown(unknown::XmacUnknownChunk::load(src, &chunk_meta)?)
                }
            }
        } else {
            println!(
                "Unknown XMAC chunk id {}.{}@{:x}, skipping",
                chunk_meta.type_id,
                chunk_meta.version,
                src.stream_position()?
            );
            XmacChunk::Unknown(unknown::XmacUnknownChunk::load(src, &chunk_meta)?)
        };
        let end_pos = src.stream_position()?;
        if end_pos != chunk_end {
            println!(
                "Chunk did not read exactly it's announced size, finished at 0x{end_pos:x} vs expected 0x{chunk_start:x} to 0x{chunk_end:x} (diff: {})!",
                end_pos - chunk_end
            );
            if chunk_meta.type_id == 3 {
                println!("--> This is known for R1 xmacs and can be ignored");
            }
        }
        Ok(result)
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        let mut data = TempWriteTarget::new(dst);
        let meta = match self {
            XmacChunk::Info(xmac_info) => xmac_info.save(&mut data, big_endian)?,
            XmacChunk::Nodes(xmac_nodes) => xmac_nodes.save(&mut data, big_endian)?,
            XmacChunk::MaterialInfo(xmac_material_info) => {
                xmac_material_info.save(&mut data, big_endian)?
            }
            XmacChunk::StdMaterial(xmac_std_material) => {
                xmac_std_material.save(&mut data, big_endian)?
            }
            XmacChunk::Mesh(xmac_mesh) => xmac_mesh.save(&mut data, big_endian)?,
            XmacChunk::SkinningInfo(xmac_skinning_info) => {
                xmac_skinning_info.save(&mut data, big_endian)?
            }
            XmacChunk::MorphTargets(xmac_morph_targets) => {
                xmac_morph_targets.save(&mut data, big_endian)?
            }
            XmacChunk::Unknown(xmac_unknown_chunk) => xmac_unknown_chunk.save(&mut data)?,
        };
        let data = data.finish();
        assert_eq!(data.len(), meta.size as usize);
        meta.save(dst, big_endian)?;
        dst.write_all(&data)?;
        Ok(())
    }

    pub fn load_unknown<R: ArchiveReadTarget>(
        parse_result: Option<XmacChunk>,
        src: &mut R,
        chunk_meta: &XmacChunkMeta,
    ) -> XmacChunk {
        parse_result.unwrap_or_else(|| {
            XmacChunk::Unknown(unknown::XmacUnknownChunk::load(src, chunk_meta).unwrap())
        })
    }

    fn get_meshes(prev_chunks: &[XmacChunk]) -> HashMap<XmacNodeId, &XmacMesh> {
        prev_chunks
            .iter()
            .filter_map(|c| {
                if let XmacChunk::Mesh(mesh) = c {
                    Some((mesh.node_id, mesh))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl XmacChunkMeta {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let type_id = read_u32_endian(src, big_endian)?;
        let size = read_u32_endian(src, big_endian)?;
        let version = read_u32_endian(src, big_endian)?;
        Ok(Self {
            type_id,
            size,
            version,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<()> {
        write_u32_endian(dst, self.type_id, big_endian)?;
        write_u32_endian(dst, self.size, big_endian)?;
        write_u32_endian(dst, self.version, big_endian)?;
        Ok(())
    }
}
