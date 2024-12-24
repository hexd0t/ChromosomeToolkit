pub mod entity;
pub mod object;
pub mod properties;
pub mod property_set;
pub mod template;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use uuid::Uuid;

use crate::error::*;
use crate::{archive::*, helpers::*};
use entity::*;
use object::*;
use property_set::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenClass {
    Invalid(String),
    DynamicLayer(Box<AccessorPropertyObject>),
    EntityDynamicContext(EntityDynamicContext),
    PropertySet(PropertySet),
    Opaque(OpaqueClass),
}

impl GenClass {
    pub fn load(src: &mut PakFile, class_name: &str, len: usize) -> Result<Self> {
        Ok(match class_name {
            "gCDynamicLayer" => {
                let mut magic = [0u8; 8];
                src.read_exact(&mut magic)?;
                assert_eq!("GENOMEDL".as_bytes(), &magic);
                let version = read_u16(src)?;
                assert_eq!(version, 0xc8);
                GenClass::DynamicLayer(Box::new(AccessorPropertyObject::load(src)?))
            }
            "eCEntityDynamicContext" => {
                GenClass::EntityDynamicContext(EntityDynamicContext::load(src)?)
            }
            "gCInventory_PS" => {
                GenClass::PropertySet(PropertySet::Inventory(Inventory::load(src)?))
            }
            "" | "INVALID" => {
                assert_eq!(len, 2);
                let inv_pad = read_u16(src)?;
                assert_eq!(inv_pad, 1);
                GenClass::Invalid(class_name.to_string())
            }
            _ => {
                println!("Warning: unknown class {class_name}");
                let mut data = vec![0; len];
                src.read_exact(&mut data)?;
                GenClass::Opaque(OpaqueClass {
                    name: class_name.to_string(),
                    data,
                })
            }
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        match self {
            GenClass::Invalid(_) => {
                let inv_pad = 1;
                write_u16(dst, inv_pad)?;
            }
            GenClass::DynamicLayer(apo) => {
                let magic = "GENOMEDL".as_bytes();
                dst.write_all(magic)?;
                let version = 0xc8;
                write_u16(dst, version)?;
                apo.save(dst)?;
            }
            GenClass::EntityDynamicContext(edc) => {
                edc.save(dst)?;
            }
            GenClass::PropertySet(set) => {
                set.save(dst)?;
            }
            GenClass::Opaque(opaque_class) => {
                dst.write_all(&opaque_class.data)?;
            }
        };
        Ok(())
    }

    pub fn get_class_name(&self) -> &str {
        match &self {
            GenClass::Invalid(name) => name.as_str(),
            GenClass::Opaque(OpaqueClass { name, data: _ }) => name.as_str(),
            GenClass::DynamicLayer(_) => "gCDynamicLayer",
            GenClass::EntityDynamicContext(_) => "eCEntityDynamicContext",
            GenClass::PropertySet(_) => "gCInventory_PS",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpaqueClass {
    name: String,
    #[serde(with = "crate::helpers::ser_hex")]
    data: Vec<u8>,
}

/// eCEntityDynamicContext
#[derive(Debug, Deserialize, Serialize)]
pub struct EntityDynamicContext {
    pub version: u16,
    pub entities: Vec<DynamicEntity>,
    pub parents: Vec<(i32, i32)>,
    pub enabled: u8,

    pub unknown1: f32,
    pub unknown2: f32,

    pub bounding_box: Option<BoundingBox>,
}

impl EntityDynamicContext {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let enabled = if version >= 2 { read_u8(src)? } else { 1 };
        let (unknown1, unknown2) = if (39..=211).contains(&version) {
            (read_f32(src)?, read_f32(src)?)
        } else {
            (0.0, 0.0)
        };
        let bounding_box = if version >= 40 {
            Some(BoundingBox::load(src)?)
        } else {
            None
        };
        let entity_count = read_u32(src)?;
        let mut entities = Vec::new();
        for _idx in 0..entity_count {
            println!("Entity start @ {:x}", src.current_read_idx + 14);
            entities.push(DynamicEntity::load(src)?);
        }

        let mut parents = Vec::new();
        loop {
            let parent = read_i32(src)?;
            let child = read_i32(src)?;
            parents.push((child, parent));
            if child == -1 && parent == -1 {
                break;
            }
        }

        Ok(Self {
            version,
            entities,
            parents,
            enabled,
            unknown1,
            unknown2,
            bounding_box,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        if self.version >= 2 {
            write_u8(dst, self.enabled)?;
        }
        if (39..=211).contains(&version) {
            write_f32(dst, self.unknown1)?;
            write_f32(dst, self.unknown2)?;
        }
        match (version >= 40, &self.bounding_box) {
            (true, Some(bb)) => {
                bb.save(dst)?;
            }
            (false, None) => {}
            (true, None) => panic!("EntityDynamicContext version >= 40 requires bounding_box!"),
            (false, Some(_)) => {
                panic!("EntityDynamicContext version < 40 must not have bounding_box!")
            }
        }
        let entity_count = self.entities.len();
        write_u32(dst, entity_count as u32)?;
        for entity in &self.entities {
            entity.save(dst)?;
        }
        assert_eq!(self.parents.last().unwrap(), &(-1, -1));
        for (child, parent) in &self.parents {
            write_i32(dst, *parent)?;
            write_i32(dst, *child)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    pub max: Vector3,
    pub min: Vector3,
}

impl BoundingBox {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            max: Vector3::load(src)?,
            min: Vector3::load(src)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        self.max.save(dst)?;
        self.min.save(dst)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sphere {
    pub radius: f32,
    pub pos: Vector3,
}

impl Sphere {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            radius: read_f32(src)?,
            pos: Vector3::load(src)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.radius)?;
        self.pos.save(dst)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

impl Matrix {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            m11: read_f32(src)?,
            m12: read_f32(src)?,
            m13: read_f32(src)?,
            m14: read_f32(src)?,

            m21: read_f32(src)?,
            m22: read_f32(src)?,
            m23: read_f32(src)?,
            m24: read_f32(src)?,

            m31: read_f32(src)?,
            m32: read_f32(src)?,
            m33: read_f32(src)?,
            m34: read_f32(src)?,

            m41: read_f32(src)?,
            m42: read_f32(src)?,
            m43: read_f32(src)?,
            m44: read_f32(src)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.m11)?;
        write_f32(dst, self.m12)?;
        write_f32(dst, self.m13)?;
        write_f32(dst, self.m14)?;

        write_f32(dst, self.m21)?;
        write_f32(dst, self.m22)?;
        write_f32(dst, self.m23)?;
        write_f32(dst, self.m24)?;

        write_f32(dst, self.m31)?;
        write_f32(dst, self.m32)?;
        write_f32(dst, self.m33)?;
        write_f32(dst, self.m34)?;

        write_f32(dst, self.m41)?;
        write_f32(dst, self.m42)?;
        write_f32(dst, self.m43)?;
        write_f32(dst, self.m44)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityProxy {
    pub version: u16,
    pub id: Option<PropertyId>,
}

impl EntityProxy {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, with_version: bool) -> Result<Self> {
        let version = if with_version { read_u16(src)? } else { 1 };
        let valid = read_u8(src)?;
        let id = if valid != 0 {
            Some(PropertyId::load(src)?)
        } else {
            None
        };
        Ok(Self { version, id })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, with_version: bool) -> Result<()> {
        if with_version {
            write_u16(dst, self.version)?;
        }
        if let Some(id) = &self.id {
            let valid = 1;
            write_u8(dst, valid)?;

            id.save(dst)?;
        } else {
            let valid = 0;
            write_u8(dst, valid)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            x: read_f32(src)?,
            y: read_f32(src)?,
        })
    }
    pub fn load_endian<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        Ok(Self {
            x: read_f32_endian(src, big_endian)?,
            y: read_f32_endian(src, big_endian)?,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.x)?;
        write_f32(dst, self.y)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            x: read_f32(src)?,
            y: read_f32(src)?,
            z: read_f32(src)?,
        })
    }
    pub fn load_endian<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        Ok(Self {
            x: read_f32_endian(src, big_endian)?,
            y: read_f32_endian(src, big_endian)?,
            z: read_f32_endian(src, big_endian)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.x)?;
        write_f32(dst, self.y)?;
        write_f32(dst, self.z)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            x: read_f32(src)?,
            y: read_f32(src)?,
            z: read_f32(src)?,
            w: read_f32(src)?,
        })
    }
    pub fn load_endian<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        Ok(Self {
            x: read_f32_endian(src, big_endian)?,
            y: read_f32_endian(src, big_endian)?,
            z: read_f32_endian(src, big_endian)?,
            w: read_f32_endian(src, big_endian)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.x)?;
        write_f32(dst, self.y)?;
        write_f32(dst, self.z)?;
        write_f32(dst, self.w)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        Ok(Self {
            x: read_f32(src)?,
            y: read_f32(src)?,
            z: read_f32(src)?,
            w: read_f32(src)?,
        })
    }
    pub fn load_endian<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        Ok(Self {
            x: read_f32_endian(src, big_endian)?,
            y: read_f32_endian(src, big_endian)?,
            z: read_f32_endian(src, big_endian)?,
            w: read_f32_endian(src, big_endian)?,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_f32(dst, self.x)?;
        write_f32(dst, self.y)?;
        write_f32(dst, self.z)?;
        write_f32(dst, self.w)?;
        Ok(())
    }
}

/// bCPropertyID
#[derive(Debug, Deserialize, Serialize)]
pub struct PropertyId {
    id: Uuid,
    unknown: u32,
}

impl PropertyId {
    pub fn load<R: ArchiveReadTarget>(src: &mut R) -> Result<Self> {
        let mut data = [0u8; 16];
        src.read_exact(&mut data)?;
        let id = Uuid::from_bytes_le(data);
        let unknown = read_u32(src)?;
        Ok(Self { id, unknown })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        dst.write_all(&self.id.to_bytes_le())?;
        // Padding:
        write_u32(dst, self.unknown)?;
        Ok(())
    }
}
/// eCNode
#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub version: u16,
}

impl Node {
    pub fn new() -> Result<Self> {
        Ok(Self { version: 210 })
    }

    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        Ok(Self { version })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DateTime(pub u64);

impl DateTime {
    pub fn load<R: Read>(src: &mut R) -> Result<Self> {
        let val = read_u64(src)?;
        Ok(Self(val))
    }
    pub fn save<W: Write>(&self, dst: &mut W) -> Result<()> {
        write_u64(dst, self.0)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Accessor {
    pub unknown: u16,
    pub object: AccessorPropertyObject,
}
impl Accessor {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let unknown = read_u16(src)?;
        let object = AccessorPropertyObject::load(src)?;
        let magic = read_u32(src)?;
        assert_eq!(magic, 0xdeadc0de);
        Ok(Self { unknown, object })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u16(dst, self.unknown)?;
        self.object.save(dst)?;
        let magic = 0xdeadc0de;
        write_u32(dst, magic)?;
        Ok(())
    }
}