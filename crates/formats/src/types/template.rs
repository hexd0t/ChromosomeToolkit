use serde::{Deserialize, Serialize};

use super::{time::DateTime, AccessorPropertyObject, Quat, Vec3};
use crate::binimport::BinImport;
use crate::error::*;
use crate::types::PropertyId;
use crate::{archive::*, helpers::*};

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplatePropertyAccessor {
    pub unknown1: u16,
    pub accessor_prop: AccessorPropertyObject,
}

impl TemplatePropertyAccessor {
    fn load(src: &mut PakFile) -> Result<Self> {
        let unknown1 = read_u16(src)?;

        let class_name = src.read_str()?.to_string();
        let accessor_size = read_u32(src)? as usize;

        println!(
            "loading '{class_name}' ({accessor_size}) @ {:08x}",
            src.current_read_idx + 14,
        );

        let accessor_start = src.current_read_idx;
        let accessor_prop = AccessorPropertyObject::load(src)?;
        if src.current_read_idx != accessor_start + accessor_size {
            println!(
                "Warning: Template offset mismatch: {} != {accessor_start} + {accessor_size} = {}",
                src.current_read_idx,
                accessor_start + accessor_size
            );
        }
        let prop_class_name = accessor_prop.object.get_class_name();
        if class_name != prop_class_name {
            return Err(Error::InvalidStructure(
                "Template class name != Object class name".to_string(),
            ));
        }

        let magic = read_u32(src)?;
        assert_eq!(magic, 0xdeadc0de);

        Ok(Self {
            unknown1,
            accessor_prop,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        write_u16(dst, self.unknown1)?;
        let class_name = self.accessor_prop.object.get_class_name();
        dst.write_str(class_name)?;

        let mut block = TempWriteTarget::new(dst);
        self.accessor_prop.save(&mut block)?;
        let block = block.finish();

        write_u32(dst, block.len() as u32)?;
        dst.write_all(&block)?;

        let magic = 0xdeadc0de;
        write_u32(dst, magic)?;

        Ok(())
    }
}

/// eCTemplateEntity
#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateEntity {
    pub version: u16,

    pub id: PropertyId,
    pub enabled: bool,
    pub rendering_enabled: bool,
    pub picking_enabled: bool,
    pub collision_enabled: bool,
    pub helper_parent: bool,
    pub is_game_relevant: bool,
    pub insert_type: u16,
    pub modified_date: DateTime,
    pub scale_grid_percentage: u8,
    pub is_savegame_relevant: bool,
    pub name: String,

    pub unknown1: bool,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown5: Vec3,
    pub unknown6: Quat,
    pub unknown7: bool,
    pub unknown8: bool,
    pub unknown9: bool,
    pub unused1: bool,
    pub unused2: bool,

    pub ref_template: Option<PropertyId>,
    pub properties: Vec<TemplatePropertyAccessor>,
}
impl TemplateEntity {
    fn load_header(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;

        let id = PropertyId::load(src)?;
        let enabled = read_u8(src)? != 0; //entity_flags 0x4 = enabled
        let rendering_enabled = read_u8(src)? != 0; //entity_flags 0x2 = RenderingEnabled
        let unused1 = read_u8(src)? != 0; //unused in R1

        if version < 215 {
            let _ = read_u8(src)?; //unused in R1
        }

        let picking_enabled = read_u8(src)? != 0; //entity_flags 0x80 = pickingEnabled
        let collision_enabled = read_u8(src)? != 0; //entity_flags 0x100 = collisionEnabled

        let (unknown1, unused2) = if version >= 219 {
            (
                read_bool(src)?, //entity_flags 0x200
                false,
            )
        } else {
            (false, read_bool(src)?) //unused in R1
        };

        let helper_parent = read_u8(src)? != 0; //flags_a 0x1

        let is_game_relevant = if version >= 214 {
            read_u8(src)? != 0 //flags_a 0x4
        } else {
            true
        };

        let has_ref_template = read_u8(src)? != 0;
        let ref_template = if has_ref_template {
            Some(PropertyId::load(src)?)
        } else {
            None
        };

        let unknown2 = read_f32(src)?;
        let unknown3 = if version >= 213 { read_f32(src)? } else { 0.0 };
        let insert_type = read_u16(src)?; //entity_flags 0x3C = insert type

        if version < 213 {
            let _ = read_u8(src)?; //unused in R1
        }

        let name = src.read_str()?.to_string();
        let unknown5 = Vec3::load(src)?;
        let unknown6 = Quat::load(src)?;
        //unknown4.normalize()

        if version < 213 {
            let _ = read_f32(src)?; //unused in R1
            let _ = read_u8(src)?; //unused in R1
        }

        if version < 212 {
            let _ = read_u8(src)?; //unused in R1
        }
        if version < 213 {
            let _ = read_f32(src)?; //unused in R1
        }

        let modified_date = DateTime::load(src)?;

        if version < 213 {
            let _ = read_u8(src)?; //unused in R1
        }
        if version < 217 {
            let _ = read_f32(src)?; //unused in R1
        }
        if version < 213 {
            let _ = read_u8(src)?; //unused in R1
            let _ = read_u8(src)?; //unused in R1
        }

        let unknown7 = read_u8(src)? != 0; //entity_flags 0x10000
        let unknown8 = read_u8(src)? != 0; //entity_flags 0x8000
        let unknown9 = read_u8(src)? != 0; //entity_flags 0x4000

        let scale_grid_percentage = if version >= 211 { read_u8(src)? } else { 100 };
        let is_savegame_relevant = if version >= 218 {
            read_u8(src)? != 0 //entity_flags 0x80000 = is savegame relevant
        } else {
            false
        };
        Ok(Self {
            version,
            ref_template,
            id,
            enabled,
            rendering_enabled,
            unused1,
            picking_enabled,
            collision_enabled,
            helper_parent,
            is_game_relevant,
            insert_type,
            unknown1,
            unknown2,
            unknown3,
            name,
            unknown5,
            unknown6,
            modified_date,
            scale_grid_percentage,
            is_savegame_relevant,
            unknown7,
            unknown8,
            unknown9,
            properties: Vec::new(),
            unused2,
        })
    }

    fn load_content(&mut self, src: &mut PakFile) -> Result<()> {
        if self.ref_template.is_some() {
            return Ok(());
        }

        let property_count = read_u32(src)? as usize;
        println!("{property_count} props @ {:08x}", src.current_read_idx + 14);
        self.properties.reserve(property_count);
        for _idx in 0..property_count {
            self.properties.push(TemplatePropertyAccessor::load(src)?);
        }

        Ok(())
    }

    pub fn load(src: &mut PakFile) -> Result<Self> {
        let mut inst = Self::load_header(src)?;
        inst.load_content(src)?;
        Ok(inst)
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        self.save_header(dst)?;
        self.save_content(dst)
    }

    fn save_header<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;

        self.id.save(dst)?;
        write_bool(dst, self.enabled)?;
        write_bool(dst, self.rendering_enabled)?;
        write_bool(dst, self.unused1)?;
        if version < 215 {
            unimplemented!("Version < 215 not supported");
        }
        write_bool(dst, self.picking_enabled)?;
        write_bool(dst, self.collision_enabled)?;

        if version >= 219 {
            write_bool(dst, self.unknown1)?;
        } else {
            write_bool(dst, self.unused2)?;
        }

        write_bool(dst, self.helper_parent)?;

        if version >= 214 {
            write_bool(dst, self.is_game_relevant)?;
        }

        if let Some(ref_template) = &self.ref_template {
            write_bool(dst, true)?;
            ref_template.save(dst)?;
        } else {
            write_bool(dst, false)?;
        }

        write_f32(dst, self.unknown2)?;
        if version >= 213 {
            write_f32(dst, self.unknown3)?;
        }
        write_u16(dst, self.insert_type)?;

        if version < 213 {
            unimplemented!("Version < 213 not supported");
        }

        dst.write_str(&self.name)?;
        self.unknown5.save(dst)?;
        self.unknown6.save(dst)?;

        if version < 213 {
            unimplemented!("Version < 213 not supported");
        }
        if version < 212 {
            unimplemented!("Version < 212 not supported");
        }
        if version < 213 {
            unimplemented!("Version < 213 not supported");
        }

        self.modified_date.save(dst)?;

        if version < 213 {
            unimplemented!("Version < 213 not supported");
        }
        if version < 217 {
            unimplemented!("Version < 217 not supported");
        }
        if version < 213 {
            unimplemented!("Version < 213 not supported");
        }

        write_bool(dst, self.unknown7)?;
        write_bool(dst, self.unknown8)?;
        write_bool(dst, self.unknown9)?;

        if version >= 211 {
            write_u8(dst, self.scale_grid_percentage)?;
        }

        if version >= 218 {
            write_bool(dst, self.is_savegame_relevant)?;
        }

        Ok(())
    }

    fn save_content<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        if self.ref_template.is_some() {
            return Ok(());
        }

        let property_count = self.properties.len();
        write_u32(dst, property_count as u32)?;
        for property in &self.properties {
            property.save(dst)?;
        }
        Ok(())
    }
}
