use serde::{Deserialize, Serialize};

use super::XmacChunkMeta;
use crate::archive::ArchiveReadTarget;
use crate::error::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacUnknownChunk {
    pub type_id: u32,
    pub version: u32,
    #[serde(with = "crate::helpers::ser_hex")]
    pub data: Vec<u8>,
}

impl XmacUnknownChunk {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, chunk_meta: &XmacChunkMeta) -> Result<Self> {
        let mut data = vec![0; chunk_meta.size as usize];
        src.read_exact(&mut data)?;
        Ok(Self {
            type_id: chunk_meta.type_id,
            version: chunk_meta.version,
            data,
        })
    }
}
