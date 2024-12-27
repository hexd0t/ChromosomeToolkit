//! Resources on the XMAC format:
//!  - O3DE (formerly Amazon Lumberyard) has an importer for XMAC, although a newer version
//!    at https://github.com/o3de/o3de/tree/development/Gems/EMotionFX/Code/EMotionFX/Source
//!    older commits contain more 'legacy' EMotionFX code
//!    (since Amazon bought EmotionFX & employs their devs, this can be considered 'ground truth')
//!    License: Apache 2 or MIT
//!  - Lumberyard's archived repo has more old pieces of EMotionFX left:
//!    at https://github.com/aws/lumberyard/blob/master/dev/Gems/EMotionFX/Code/EMotionFX/Source/Importer/ActorFileFormat.h
//!    License: AWS Agreement
//!  - RisenEditor:
//!    at https://github.com/hhergeth/RisenEditor
//!    License: none
//!  - Baltram's rmTools:
//!    at https://github.com/Baltram/rmtools/blob/master/rmStuff/rmXmacReader.cpp
//!    License: GPLv3

pub mod chunks;

use serde::{Deserialize, Serialize};

use crate::{
    archive::ArchiveReadTarget,
    error::*,
    helpers::*,
    types::{properties::Property, DateTime},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacFile {
    pub timestamp: DateTime,
    pub props: Vec<Property>,
    pub chunks: Vec<chunks::XmacChunk>,
}

impl XmacFile {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let mut revision = [0u8; 4];
        src.read_exact(&mut revision)?;
        assert_eq!(&revision, "GR01".as_bytes());
        let mut data_revision = [0u8; 4];
        src.read_exact(&mut data_revision)?;
        assert_eq!(&data_revision, "MA02".as_bytes());

        //prop_offset: u32 - Header up to that point is always 0x28
        let prop_offset = read_u32(src)? as usize;
        assert_eq!(prop_offset, 0x28);

        let prop_length = read_u32(src)? as usize;

        let data_offset = read_u32(src)? as usize;
        assert_eq!(prop_length + prop_offset, data_offset);

        let _data_len = read_u32(src)? as usize;

        let timestamp = DateTime::load(src)?;

        let mut raw_file_ext = [0u8; 8];
        src.read_exact(&mut raw_file_ext)?;
        assert_eq!(&raw_file_ext[0..4], ".xac".as_bytes());
        assert_eq!(&raw_file_ext[4..8], &[0, 0, 0, 0]);

        let props = Self::load_meta(src)?;
        let chunks = Self::load_xmac(src)?;

        let trail = read_u64(src)?;
        assert_eq!(trail, 0);

        Ok(XmacFile {
            props,
            chunks,
            timestamp,
        })
    }

    fn load_meta<R: ArchiveReadTarget>(src: &mut R) -> Result<Vec<Property>> {
        let mut header = [0u8; 6];
        src.read_exact(&mut header)?;
        assert_eq!(&header, &[1, 0, 1, 1, 0, 1]);

        let class_name = src.read_str()?;
        assert_eq!(class_name.as_str(), "eCMotionActorResource2");
        let mut unknown1 = [0u8; 3];
        src.read_exact(&mut unknown1)?;
        assert_eq!(&unknown1, &[1, 0, 0]);

        let class_ver = read_u16(src)?;
        assert_eq!(class_ver, 201);
        let version = read_u16(src)?;
        assert_eq!(version, 201);
        let _data_len = read_u32(src)?;

        let prop_data_ver = read_u16(src)?;
        assert_eq!(prop_data_ver, 201);
        let prop_count = read_u32(src)? as usize;
        let mut props = Vec::with_capacity(prop_count);
        for _idx in 0..prop_count {
            props.push(Property::load(src)?);
        }
        let class_version = read_u16(src)?;
        assert_eq!(class_version, 201);
        Ok(props)
    }

    fn load_xmac<R: ArchiveReadTarget>(src: &mut R) -> Result<Vec<chunks::XmacChunk>> {
        let data_len = read_u32(src)? as u64;
        let xmac_start = src.stream_position()?;
        let xmac_finish = xmac_start + data_len;

        let mut magic = vec![0; 4];
        src.read_exact(&mut magic)?;
        assert_eq!(&magic, "XAC ".as_bytes());

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

        Ok(chunks)
    }
}

/// XMAC Strings store their length in (endianness-affected) u32
fn read_xmac_str<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<String> {
    let len = read_u32_endian(src, big_endian)? as usize;
    if len > 255 {
        return Err(Error::InvalidStructure(format!(
            "String @{:x} is supposedly {len} bytes",
            src.stream_position()?
        )));
    }
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
