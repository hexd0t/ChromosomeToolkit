use std::io::Cursor;

use serde::Deserialize;
use serde::Serialize;

use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::properties::Property;
use crate::types::time::DateTime;

//Header up to that point is always 0x28
const PROP_OFFSET: u32 = 0x28;
const PROP_VERSION: u16 = 201;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceFile {
    pub timestamp: DateTime,
    pub props: Vec<Property>,
    pub data_revision: [u8; 4],
    pub class_name: String,
    pub raw_file_ext: [u8; 8],
}

impl ResourceFile {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let mut revision = [0u8; 4];
        src.read_exact(&mut revision)?;
        assert_eq!(&revision, "GR01".as_bytes());
        let mut data_revision = [0u8; 4];
        src.read_exact(&mut data_revision)?;

        let prop_offset = read_u32(src)?;
        assert_eq!(prop_offset, PROP_OFFSET);

        let prop_length = read_u32(src)?;

        let data_offset = read_u32(src)?;
        assert_eq!(prop_length + prop_offset, data_offset);

        let _data_len = read_u32(src)?;

        let timestamp = DateTime::load(src)?;

        let mut raw_file_ext = [0u8; 8];
        src.read_exact(&mut raw_file_ext)?;

        let (class_name, props) = Self::load_props(src)?;

        Ok(Self {
            timestamp,
            props,
            data_revision,
            class_name,
            raw_file_ext,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, data_len: usize) -> Result<()> {
        let mut props_buf = Cursor::new(Vec::<u8>::new());
        self.save_props(&mut props_buf)?;
        let props = props_buf.into_inner();
        //res_magic: [u8; 4], // "GR01"
        dst.write_all("GR01".as_bytes())?;
        //res_class: [u8; 4]
        dst.write_all(&self.data_revision)?;
        //prop_offset: u32 - Header up to that point is always 0x28
        write_u32(dst, PROP_OFFSET)?;
        //prop_length: u32,
        write_u32(dst, props.len() as u32)?;
        //data_offset: u32,
        write_u32(dst, PROP_OFFSET + props.len() as u32)?;
        //data_length: u32,
        write_u32(dst, data_len as u32)?;
        //raw_timestamp: u64,    // (FILETIME)
        write_u64(dst, self.timestamp.0)?;
        //raw_file_ext: [u8; 8], // ".wav"*/
        dst.write_all(&self.raw_file_ext)?;

        dst.write_all(&props)?;
        Ok(())
    }

    fn load_props<R: ArchiveReadTarget>(src: &mut R) -> Result<(String, Vec<Property>)> {
        let mut header = [0u8; 6];
        src.read_exact(&mut header)?;
        assert_eq!(&header, &[1, 0, 1, 1, 0, 1]);

        let class_name = src.read_str()?;
        let mut unknown1 = [0u8; 3];
        src.read_exact(&mut unknown1)?;
        assert_eq!(&unknown1, &[1, 0, 0]);

        let class_ver = read_u16(src)?;
        assert_eq!(class_ver, PROP_VERSION);
        let version = read_u16(src)?;
        assert_eq!(version, PROP_VERSION);
        let _data_len = read_u32(src)?; //TODO: verify

        let prop_data_ver = read_u16(src)?;
        assert_eq!(prop_data_ver, PROP_VERSION);
        let prop_count = read_u32(src)? as usize;
        let mut props = Vec::with_capacity(prop_count);
        for _idx in 0..prop_count {
            props.push(Property::load(src)?);
        }
        let class_version = read_u16(src)?;
        assert_eq!(class_version, PROP_VERSION);
        Ok((class_name, props))
    }

    fn save_props<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let mut props_data_buf = Cursor::new(Vec::<u8>::new());
        for prop in &self.props {
            prop.save(&mut props_data_buf)?;
        }
        let props_data = props_data_buf.into_inner();

        // Header:
        dst.write_all(&[1, 0, 1, 1, 0, 1])?;
        // ClassName:
        dst.write_str(&self.class_name)?;
        // unknown:
        dst.write_all(&[1, 0, 0])?;
        // class version:
        write_u16(dst, PROP_VERSION)?;
        // version:
        write_u16(dst, PROP_VERSION)?;
        // data size:
        let data_size: usize = 2 + 4 + props_data.len() + 2;
        write_u32(dst, data_size as u32)?;
        // data version:
        write_u16(dst, PROP_VERSION)?;
        // property count:
        write_u32(dst, self.props.len() as u32)?;
        // property data:
        dst.write_all(&props_data)?;

        // class version:
        write_u16(dst, PROP_VERSION)?;

        Ok(())
    }

    // fn write_prop(name: &str, p_type: &str, data: &[u8]) -> Vec<u8> {
    //     let size = 2 + 2 + 2 + 4 + name.len() + p_type.len() + data.len();
    //     let mut w = Cursor::new(Vec::with_capacity(size));
    //     write_u16(dst, name.len() as u16)?;
    //     dst.write_all(name.as_bytes())?;
    //     write_u16(dst, p_type.len() as u16)?;
    //     dst.write_all(p_type.as_bytes())?;
    //     write_u16(dst, 0x1E)?;
    //     write_u32(dst, data.len() as u32)?;
    //     dst.write_all(data)?;
    //     dst.into_inner()
    // }
}
