use serde::{Deserialize, Serialize};

use super::XmacChunkMeta;

use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMaterialInfo {
    pub std_materials: usize,
    /// might also be generic materials instead
    pub fx_materials: usize,
}

impl XmacMaterialInfo {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading MATERIAL INFO chunk...");
        match chunk_meta.version {
            1 => {
                let total_materials = read_u32_endian(src, big_endian)? as usize;
                let std_materials = read_u32_endian(src, big_endian)? as usize;
                let fx_materials = read_u32_endian(src, big_endian)? as usize;

                if total_materials != std_materials {
                    return Err(Error::InvalidStructure(
                        "Non Std-Materials are not supported right now".to_string(),
                    ));
                }

                Ok(Some(Self {
                    std_materials,
                    fx_materials,
                }))
            }
            ver => {
                println!(
                    "Unknown XMAC material info version {ver}@{:x}, skipping",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
}
