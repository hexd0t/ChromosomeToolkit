use super::super::read_xmac_str;
use super::XmacChunkMeta;

use serde::{Deserialize, Serialize};

use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacInfo {
    // TODO: The version used by R1 carries only one of these:
    /// the node number of the trajectory node used for motion extraction
    /// (always 1 or 0 for R1)
    pub unknown1: i32,
    /// the retargeting root node index, most likely pointing to the hip or pelvis or invalid index (-1) when not set
    /// (always -1 for R1)
    pub retarget_root_node_index: i32,
    /// Major Version of the Exporting Software
    /// (always 3 for R1)
    pub exporter_maj: u8,
    /// Minor Version of the Exporting Software
    /// (0 or 6 for R1)
    pub exporter_min: u8,
    // (always 0 for R1)
    pub unknown2: u16,
    // (always 0 for R1)
    pub unknown3: u32,
    /// source application (e.g. "3ds Max 2011", "Maya 2011")
    pub source_application: String,
    /// original filename of the 3dsMax/Maya file
    pub orig_filename: String,
    /// compilation date of the exporter
    pub exporter_date: String,
    /// the name of the actor (always "" for R1)
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
                unknown1: read_i32_endian(src, big_endian)?,
                retarget_root_node_index: read_i32_endian(src, big_endian)?,
                exporter_maj: read_u8(src)?,
                exporter_min: read_u8(src)?,
                unknown2: read_u16_endian(src, big_endian)?,
                unknown3: read_u32_endian(src, big_endian)?,
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
    pub fn new(source_application: String, orig_filename: String) -> Self {
        Self {
            unknown1: 0,
            retarget_root_node_index: -1,
            exporter_maj: 0,
            exporter_min: 1,
            unknown2: 0,
            unknown3: 0,
            source_application,
            orig_filename,
            exporter_date: env!("CARGO_PKG_VERSION").to_string(),
            actor_name: "".to_string(),
        }
    }
}
