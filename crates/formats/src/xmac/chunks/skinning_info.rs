use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::mesh::XmacMesh;
use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use super::XmacChunkType;
use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacSkinningInfo {
    pub node_id: XmacNodeId,

    pub influences: Vec<SkinInfluence>,
    pub table_entries: Vec<TableEntry>,

    pub local_bones: u32,
    pub is_for_collision_mesh: bool,
    pub unknown1: [u8; 3],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkinInfluence {
    pub weight: f32,
    pub node_idx: u16,
    /// Always the same for all Influences in a skin
    pub unknown: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableEntry {
    /// index into SkinningInfo.influences for the NativeVertex at this idx
    pub start_idx: u32,
    /// number of influences for this entry
    pub num_elements: u32,
}

impl XmacSkinningInfo {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
        meshes: &HashMap<XmacNodeId, &XmacMesh>,
    ) -> Result<Option<Self>> {
        println!("Loading SKINNING INFO chunk...");
        match chunk_meta.version {
            3 => {
                let node_id = XmacNodeId(read_u32_endian(src, big_endian)?);
                let local_bones = read_u32_endian(src, big_endian)?;
                let total_influences = read_u32_endian(src, big_endian)?;
                let is_for_collision_mesh = read_bool(src)?;

                let unknown1 = [read_u8(src)?, read_u8(src)?, read_u8(src)?];

                let mut influences = Vec::with_capacity(total_influences as usize);
                for _idx in 0..total_influences {
                    influences.push(SkinInfluence::load(src, big_endian)?);
                }

                let mesh_orig_vertices_count = if let Some(mesh) = meshes.get(&node_id) {
                    mesh.orig_verts_count
                } else {
                    return Err(Error::InvalidStructure(format!(
                        "Found skinning info for nonexistent mesh {node_id:?}!"
                    )));
                };
                let mut table_entries = Vec::with_capacity(mesh_orig_vertices_count as usize);
                for _idx in 0..mesh_orig_vertices_count {
                    table_entries.push(TableEntry::load(src, big_endian)?);
                }

                Ok(Some(Self {
                    node_id,
                    influences,
                    table_entries,
                    local_bones,
                    is_for_collision_mesh,
                    unknown1,
                }))
            }
            ver => {
                println!(
                    "Unknown XMAC skinning info version {ver}@{:x}",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
    pub fn save<W: ArchiveWriteTarget>(
        &self,
        dst: &mut W,
        big_endian: bool,
    ) -> Result<XmacChunkMeta> {
        println!("Saving SKINNING INFO chunk...");
        write_u32_endian(dst, self.node_id.0, big_endian)?;
        write_u32_endian(dst, self.local_bones, big_endian)?;
        write_u32_endian(dst, self.influences.len() as u32, big_endian)?;
        write_bool(dst, self.is_for_collision_mesh)?;
        dst.write_all(&self.unknown1)?;
        let mut written = 4 + 4 + 4 + 1 + 3;

        for influence in &self.influences {
            written += influence.save(dst, big_endian)?;
        }

        for table_entry in &self.table_entries {
            written += table_entry.save(dst, big_endian)?;
        }

        Ok(XmacChunkMeta {
            type_id: XmacChunkType::SkinningInfo.into(),
            size: written as u32,
            version: 3,
        })
    }
}

impl SkinInfluence {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let weight = read_f32_endian(src, big_endian)?;
        let node_idx = read_u16_endian(src, big_endian)?;
        let unknown = read_u16_endian(src, big_endian)?;
        Ok(Self {
            weight,
            node_idx,
            unknown,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        write_f32_endian(dst, self.weight, big_endian)?;
        write_u16_endian(dst, self.node_idx, big_endian)?;
        write_u16_endian(dst, self.unknown, big_endian)?;

        Ok(4 + 2 + 2)
    }
}

impl TableEntry {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let start_idx = read_u32_endian(src, big_endian)?;
        let num_elements = read_u32_endian(src, big_endian)?;
        Ok(Self {
            start_idx,
            num_elements,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        write_u32_endian(dst, self.start_idx, big_endian)?;
        write_u32_endian(dst, self.num_elements, big_endian)?;

        Ok(4 + 4)
    }
}
