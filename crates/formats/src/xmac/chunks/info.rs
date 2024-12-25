use super::super::read_xmac_str;
use super::XmacChunkMeta;

use serde::{Deserialize, Serialize};

use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacInfo {
    /// the number of level of details - R1 doesn't seem to have LOD fields though
    // pub lod_count: u32,
    // TODO: The version used by R1 carries only one of these:
    /// the node number of the trajectory node used for motion extraction
    pub motion_extraction_node_index: i32,
    /// the retargeting root node index, most likely pointing to the hip or pelvis or invalid index (.1) when not set
    pub retarget_root_node_index: i32,
    /// supposedly contains unit_type (feet, cm, m, ...) and exporter version, but none of those are used
    pub unknown1: u32,
    pub unknown2: u32,
    /// source application (e.g. "3ds Max 2011", "Maya 2011")
    pub source_application: String,
    /// original filename of the 3dsMax/Maya file
    pub orig_filename: String,
    /// compilation date of the exporter
    pub exporter_date: String,
    /// the name of the actor
    pub actor_name: String,
}

impl XmacInfo {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading INFO chunk...");
        match chunk_meta.version {
            2 => Ok(Some(XmacInfo {
                //lod_count: read_u32_endian(src, big_endian)?,
                motion_extraction_node_index: read_i32_endian(src, big_endian)?,
                retarget_root_node_index: read_i32_endian(src, big_endian)?,
                unknown1: read_u32_endian(src, big_endian)?,
                unknown2: read_u32_endian(src, big_endian)?,
                source_application: read_xmac_str(src, big_endian)?,
                orig_filename: read_xmac_str(src, big_endian)?,
                exporter_date: read_xmac_str(src, big_endian)?,
                actor_name: read_xmac_str(src, big_endian)?,
            })),
            ver => {
                println!(
                    "Unknown XMAC info version {ver}@{:x}",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}
