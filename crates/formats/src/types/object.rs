use serde::{Deserialize, Serialize};

use super::*;
use crate::archive::*;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
/// bCObjectBase
pub struct Object {
    pub version: u16,
    pub class: GenClass,
    pub prop_data_ver: u16,
    #[serde(default = "Vec::new")]
    pub props: Vec<properties::Property>,
}

impl Object {
    pub fn load(src: &mut PakFile, class_name: &str) -> Result<Self> {
        let version = read_u16(src)?;

        let object_data_size = read_u32(src)? as usize;
        let prop_start_off = src.current_read_idx;
        let prop_data_ver = read_u16(src)?;
        assert_eq!(prop_data_ver, 201);
        let prop_count = read_u32(src)? as usize;
        let mut props = Vec::with_capacity(prop_count);
        for _idx in 0..prop_count {
            props.push(properties::Property::load(src)?);
        }
        println!("Loading class {class_name}");
        let class_len = prop_start_off + object_data_size - src.current_read_idx;

        let class = GenClass::load(src, class_name, class_len)?;
        if src.current_read_idx != prop_start_off + object_data_size {
            println!(
                "Warning: read idx {} != {} + {} = {}",
                src.current_read_idx,
                prop_start_off,
                object_data_size,
                prop_start_off + object_data_size
            );
            src.current_read_idx = prop_start_off + object_data_size;
        }
        println!("Class {class_name} loaded");

        Ok(Self {
            version,
            prop_data_ver,
            props,
            class,
        })
    }

    pub fn new() -> Self {
        Self {
            version: 210,
            class: GenClass::Invalid("INVALID".to_string()),
            prop_data_ver: 201,
            props: Vec::new(),
        }
    }

    pub fn get_class_name(&self) -> &str {
        self.class.get_class_name()
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u16(dst, self.version)?;

        let mut block = TempWriteTarget::new(dst);

        write_u16(&mut block, self.prop_data_ver)?;

        let prop_count = self.props.len();
        write_u32(&mut block, prop_count as u32)?;
        for prop in &self.props {
            prop.save(&mut block)?;
        }

        self.class.save(&mut block)?;

        let prop_data = block.finish();
        write_u32(dst, prop_data.len() as u32)?;
        dst.write_all(&prop_data)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// bCAccessorPropertyObject
pub struct AccessorPropertyObject {
    pub version: u16,
    pub object: Object,
    pub valid: u8,
    pub object_version: u16,

    pub unknown1: u16,
    pub unknown2: u8,
    pub unknown3: u16,
    pub unknown4: u8,
}

impl AccessorPropertyObject {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        if version != 1 {
            return Err(Error::UnknownVersion(format!(
                "Unknown AccessorPropertyObject Version {version}, expected 1"
            )));
        }
        let valid = read_u8(src)?;
        Ok(if valid != 0 {
            let unknown1 = read_u16(src)?;
            let unknown2 = read_u8(src)?;
            let class_name = src.read_str()?.to_string();
            let unknown3 = read_u16(src)?;
            let unknown4 = read_u8(src)?;
            let object_version = read_u16(src)?;

            let object = Object::load(src, &class_name)?;

            Self {
                version,
                valid,
                object_version,
                object,
                unknown1,
                unknown2,
                unknown3,
                unknown4,
            }
        } else {
            Self {
                valid,
                ..Self::new()
            }
        })
    }

    pub fn new() -> Self {
        Self {
            version: 1,
            valid: 1,
            object_version: 1,
            object: Object::new(),
            unknown1: 1,
            unknown2: 1,
            unknown3: 1,
            unknown4: 0,
        }
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u16(dst, self.version)?;
        write_u8(dst, self.valid)?;
        if self.valid != 0 {
            write_u16(dst, self.unknown1)?;
            write_u8(dst, self.unknown2)?;
            dst.write_str(self.object.get_class_name())?;
            write_u16(dst, self.unknown3)?;
            write_u8(dst, self.unknown4)?;
            write_u16(dst, self.object_version)?;

            self.object.save(dst)?;
        }
        Ok(())
    }
}
