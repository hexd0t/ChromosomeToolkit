use containers::RefPtrArray;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use super::*;
use crate::archive::*;
use crate::error::*;
use crate::helpers::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum PropertySet {
    Inventory(Inventory),
    Interaction(Interaction),
    Party(Party),
    Animation(Animation),
    Mesh(Mesh),
    Anchor(Anchor),
}

impl PropertySet {
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        match self {
            PropertySet::Inventory(inventory) => inventory.save(dst),
            PropertySet::Interaction(interaction) => interaction.save(dst),
            PropertySet::Party(party) => party.save(dst),
            PropertySet::Mesh(mesh) => mesh.save(dst),
            PropertySet::Anchor(anchor) => anchor.save(dst),
            PropertySet::Animation(animation) => animation.save(dst),
        }
    }

    pub(crate) fn get_class_name(&self) -> &str {
        match self {
            PropertySet::Inventory(_) => "gCInventory_PS",
            PropertySet::Interaction(_) => "gCInteraction_PS",
            PropertySet::Party(_) => "gCParty_PS",
            PropertySet::Animation(_) => "eCAnimation_PS",
            PropertySet::Mesh(_) => "eCMesh_PS",
            PropertySet::Anchor(_) => "gCAnchor_PS",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// gCInventory_PS
pub struct Inventory {
    pub version: u16,
    pub unknown1: u16,
    pub unknown2: u8,
    pub unknown3: u16,
    pub stacks: Vec<Object>,
    pub slots: Vec<InventorySlot>,
}

impl Inventory {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let unknown1 = read_u16(src)?;

        let unknown2 = read_u8(src)?;
        let stack_count = read_u32(src)? as usize;
        let mut stacks = Vec::with_capacity(stack_count);
        for _idx in 0..stack_count {
            stacks.push(Object::load(src, "INVALID")?);
        }
        let unknown3 = read_u16(src)?;
        let slot_count = read_u32(src)? as usize;
        let mut slots = Vec::with_capacity(slot_count);
        for _idx in 0..slot_count {
            slots.push(InventorySlot::load(src)?);
        }
        Ok(Self {
            version,
            unknown1,
            unknown2,
            unknown3,
            stacks,
            slots,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        write_u16(dst, self.unknown1)?;
        write_u8(dst, self.unknown2)?;

        let stack_count = self.stacks.len();
        write_u32(dst, stack_count as u32)?;
        for stack in &self.stacks {
            stack.save(dst)?;
        }

        write_u16(dst, self.unknown3)?;

        let slot_count = self.slots.len();
        write_u32(dst, slot_count as u32)?;
        for slot in &self.slots {
            slot.save(dst)?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InventorySlot {
    pub id: InventorySlotIdx,
    pub unknown1: [u8; 3],
    pub content: Object,
}

impl InventorySlot {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let id = InventorySlotIdx::try_from(read_u8(src)?)?;
        let mut unknown1 = [0u8; 3];
        src.read_exact(&mut unknown1)?;

        let content = Object::load(src, "INVALID")?;
        Ok(Self {
            id,
            unknown1,
            content,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u8(dst, self.id.into())?;
        dst.write_all(&self.unknown1)?;
        self.content.save(dst)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum InventorySlotIdx {
    None = 0,
    Head = 10,
    Body = 11,
    Helmet = 12,
}

#[derive(Debug, Deserialize, Serialize)]
/// gCInteraction_PS
pub struct Interaction {
    pub version: u16,
    pub data: Vec<AccessorPropertyObject>,
}

impl Interaction {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;

        let data_count = read_u32(src)? as usize;
        let mut data = Vec::with_capacity(data_count);
        for _idx in 0..data_count {
            data.push(AccessorPropertyObject::load(src)?);
        }

        Ok(Self { version, data })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;

        let data_count = self.data.len();
        write_u32(dst, data_count as u32)?;
        for obj in &self.data {
            obj.save(dst)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// gCParty_PS
pub struct Party {
    pub version: u16,
    pub unknown1: String,
    pub unknown2: bool,
    pub proxy: EntityProxy,
}

impl Party {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;

        let unknown1 = src.read_str()?;
        let unknown2 = read_bool(src)?;

        let proxy = EntityProxy::load(src, true)?;

        Ok(Self {
            version,
            unknown1,
            unknown2,
            proxy,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;

        dst.write_str(&self.unknown1)?;
        write_bool(dst, self.unknown2)?;

        self.proxy.save(dst, true)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// eCAnimation_PS
pub struct Animation {
    #[serde(flatten)]
    pub properties: EntityPropertySet,
}

impl Animation {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        Ok(Self {
            properties: EntityPropertySet::load(src)?,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        self.properties.save(dst)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// eCMesh_PS
pub struct Mesh {
    pub version: u16,
    pub bounding: Option<BoundingBox>,
    pub entity_ps: EntityPropertySet,
}

impl Mesh {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let bounding = if version == 2 {
            Some(BoundingBox::load(src)?)
        } else {
            None
        };
        let entity_ps = EntityPropertySet::load(src)?;
        Ok(Self {
            version,
            bounding,
            entity_ps,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;

        match (version, &self.bounding) {
            (2, Some(bb)) => bb.save(dst)?,
            (2, None) => Err(Error::InvalidStructure(
                "Mesh Version 2 must have a BoundingBox!".to_string(),
            ))?,
            (_, Some(_)) => Err(Error::InvalidStructure(
                "Mesh Version != 2 can not have a BoundingBox!".to_string(),
            ))?,
            (_, None) => {}
        }

        self.entity_ps.save(dst)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// gCAnchor_PS
pub struct Anchor {
    pub version: u16,
    pub entities: RefPtrArray<EntityProxy>,
}

impl Anchor {
    pub fn load(src: &mut PakFile, len: usize) -> Result<Self> {
        let version = read_u16(src)?;
        let entities = if len > 2 {
            RefPtrArray::load(src)?
        } else {
            RefPtrArray(Vec::new())
        };
        Ok(Self { version, entities })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        self.entities.save(dst)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// eCEntityPropertySet
pub struct EntityPropertySet {
    pub version: u16,
    pub unknown1: u8,
}

impl EntityPropertySet {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let unknown1 = if version > 1 { read_u8(src)? } else { 0 };
        Ok(Self { version, unknown1 })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;

        if version > 1 {
            write_u8(dst, self.unknown1)?;
        }

        Ok(())
    }
}
