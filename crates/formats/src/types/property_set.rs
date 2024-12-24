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
}

impl PropertySet {
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        match self {
            PropertySet::Inventory(inventory) => inventory.save(dst),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// gCInventory_PS
pub struct Inventory {
    pub version: u16,
    pub version2: u16,
    pub version3: u8,
    pub version4: u16,
    pub stacks: Vec<Object>,
    pub slots: Vec<InventorySlot>,
}

impl Inventory {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let version2 = read_u16(src)?;

        let version3 = read_u8(src)?;
        let stack_count = read_u32(src)? as usize;
        let mut stacks = Vec::with_capacity(stack_count);
        for _idx in 0..stack_count {
            stacks.push(Object::load(src, "INVALID")?);
        }
        let version4 = read_u16(src)?;
        let slot_count = read_u32(src)? as usize;
        let mut slots = Vec::with_capacity(slot_count);
        for _idx in 0..slot_count {
            slots.push(InventorySlot::load(src)?);
        }
        Ok(Self {
            version,
            version2,
            version3,
            version4,
            stacks,
            slots,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        write_u16(dst, self.version2)?;
        write_u8(dst, self.version3)?;

        let stack_count = self.stacks.len();
        write_u32(dst, stack_count as u32)?;
        for stack in &self.stacks {
            stack.save(dst)?;
        }

        write_u16(dst, self.version4)?;

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
    pub content: Object,
}

impl InventorySlot {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let id = InventorySlotIdx::try_from(read_u32(src)?)?;
        let content = Object::load(src, "INVALID")?;
        Ok(Self { id, content })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u32(dst, self.id.into())?;
        self.content.save(dst)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, IntoPrimitive, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum InventorySlotIdx {
    Head = 10,
    Body = 11,
    Helmet = 12,
}
