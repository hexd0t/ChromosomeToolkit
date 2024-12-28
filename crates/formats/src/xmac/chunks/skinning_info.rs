use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::mesh::XmacMesh;
use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacSkinningInfo {
    pub node_id: XmacNodeId,

    pub influences: Vec<SkinInfluence>,
    pub table_entries: Vec<TableEntry>,

    pub local_bones: u32,
    pub unknown: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkinInfluence {
    pub weight: f32,
    pub node_idx: u16,
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
                //let is_for_collision_mesh = read_bool(src)?;
                let unknown = read_u32_endian(src, big_endian)?;

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
                    unknown,
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
}
