use serde::{Deserialize, Serialize};
use std::io::Read;
use uuid::Uuid;

use super::*;
use crate::error::*;
use crate::{archive::*, helpers::*};

/// eCDynamicEntity
#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicEntity {
    pub version: u16,
    pub version2: u16,

    pub id: Uuid,
    pub name: String,
    pub unknown1: u32,
    pub creator: EntityProxy,

    pub local_matrix: Matrix,
    pub world_bound: BoundingBox,
    pub world_sphere: Sphere,
    pub local_bound: BoundingBox,

    pub geo_entity: GeometryEntity,
}

impl DynamicEntity {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        println!("ver: {version}");
        let version2 = if version <= 210 { read_u16(src)? } else { 0 };
        let mut id = [0u8; 16];
        src.read_exact(&mut id)?;
        let id = Uuid::from_bytes_le(id);
        let unknown1 = read_u32(src)?;
        let name = src.read_str()?.to_string();
        println!("name: {name}");
        let local_matrix = Matrix::load(src)?;
        let world_bound = BoundingBox::load(src)?;
        let world_sphere = Sphere::load(src)?;
        let local_bound = BoundingBox::load(src)?;
        let creator = EntityProxy::load(src, version >= 213)?;
        let geo_entity = GeometryEntity::load(src)?;

        Ok(Self {
            version,
            version2,
            id,
            name,
            creator,
            unknown1,
            local_matrix,
            world_bound,
            world_sphere,
            local_bound,
            geo_entity,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        if version <= 210 {
            write_u16(dst, self.version2)?;
        }
        let id_bytes = self.id.to_bytes_le();
        dst.write_all(&id_bytes)?;
        write_u32(dst, self.unknown1)?;
        dst.write_str(&self.name)?;
        self.local_matrix.save(dst)?;
        self.world_bound.save(dst)?;
        self.world_sphere.save(dst)?;
        self.local_bound.save(dst)?;
        self.creator.save(dst, version >= 213)?;
        self.geo_entity.save(dst)?;
        Ok(())
    }
}

/// eCGeometryEntity
#[derive(Debug, Deserialize, Serialize)]
pub struct GeometryEntity {
    pub version: u16,
    pub unknown1: f32,
    pub geo_matrix: Matrix,
    pub geo_bound: BoundingBox,
    pub geo_sphere: Sphere,
    pub alpha: f32,
    pub view_range: f32,
    pub cache_in_range: f32,
    pub unknown2: [u8; 16],
    pub entity: Entity,
}
impl GeometryEntity {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let unknown1 = if version <= 213 { read_f32(src)? } else { 0.0 };
        let geo_matrix = Matrix::load(src)?;
        let geo_bound = BoundingBox::load(src)?;
        let geo_sphere = Sphere::load(src)?;
        let (alpha, view_range, unknown2) = if version >= 213 {
            (read_f32(src)?, read_f32(src)?, [0; 16])
        } else {
            let mut buf = [0; 16];
            src.read_exact(&mut buf)?;
            (0.0, 0.0, buf)
        };
        let cache_in_range = if version >= 214 { read_f32(src)? } else { 0.0 };
        let entity = Entity::load(src)?;
        Ok(Self {
            version,
            unknown1,
            geo_matrix,
            geo_bound,
            geo_sphere,
            alpha,
            view_range,
            cache_in_range,
            unknown2,
            entity,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        if version <= 213 {
            write_f32(dst, self.unknown1)?;
        }
        self.geo_matrix.save(dst)?;
        self.geo_bound.save(dst)?;
        self.geo_sphere.save(dst)?;
        if version >= 213 {
            write_f32(dst, self.alpha)?;
            write_f32(dst, self.view_range)?;
        } else {
            dst.write_all(&self.unknown2)?;
        }
        if version >= 214 {
            write_f32(dst, self.cache_in_range)?;
        }
        self.entity.save(dst)?;
        Ok(())
    }
}

/// eCEntity
#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub version: u16,
    pub node: Node,
    pub enabled: u8,
    pub render_enabled: u8,
    pub picking_enabled: u8,
    pub collision_enabled: u8,
    pub insert_type: u16,
    pub locked: u8,
    pub changed_timestamp: DateTime,
    pub is_savegame_relevant: u8,

    #[serde(default = "Vec::new")]
    pub accessors: Vec<Accessor>,

    pub flag_a: bool,
    pub flag_b: bool,
    pub flag_c: bool,
    pub flag_d: bool,
    pub flag_e: bool,

    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u8,
    pub unknown5: u8,
    pub unknown6: u8,
    pub unknown7: u8,
}

impl Entity {
    pub fn load(src: &mut PakFile) -> Result<Self> {
        let version = read_u16(src)?;
        let node = Node::load(src)?;
        let enabled = read_u8(src)?;
        let render_enabled = read_u8(src)?;
        let unknown1 = read_u8(src)?;
        let (flag_a, flag_b, flag_c) = if version >= 211 {
            (
                read_bool(src)?, // 4000 in EntityFlags
                read_bool(src)?, // 8000 in EntityFlags
                read_bool(src)?, // 10000 in EntityFlags
            )
        } else {
            (true, false, true)
        };
        let unknown2 = if version <= 213 { read_u8(src)? } else { 0 };
        let unknown3 = read_u8(src)?;
        let picking_enabled = read_u8(src)?;
        let collision_enabled = read_u8(src)?;
        let insert_type = read_u16(src)?;
        let locked = read_u8(src)?;
        let (unknown4, flag_d, unknown5) = if version <= 212 {
            (
                read_u8(src)?,
                read_bool(src)?, // 200 in EntityFlags
                read_u8(src)?,
            )
        } else {
            (0, read_bool(src)?, 0)
        };

        let changed_timestamp = DateTime::load(src)?;
        let (unknown6, unknown7) = if version <= 212 {
            (read_u8(src)?, read_u8(src)?)
        } else {
            (0, 0)
        };
        let flag_e = if version >= 212 {
            read_bool(src)? // 40000 in EntityFlags
        } else {
            false
        };
        let is_savegame_relevant = if version >= 216 { read_u8(src)? } else { 1 };

        let accessor_count = read_u32(src)? as usize;
        let mut accessors = Vec::with_capacity(accessor_count);
        for _idx in 0..accessor_count {
            accessors.push(Accessor::load(src)?);
        }

        Ok(Self {
            version,
            node,
            enabled,
            render_enabled,
            picking_enabled,
            collision_enabled,
            insert_type,
            locked,
            changed_timestamp,
            is_savegame_relevant,
            accessors,
            flag_a,
            flag_b,
            flag_c,
            flag_d,
            flag_e,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
            unknown6,
            unknown7,
        })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W) -> Result<()> {
        let version = self.version;
        write_u16(dst, version)?;
        self.node.save(dst)?;
        write_u8(dst, self.enabled)?;
        write_u8(dst, self.render_enabled)?;
        write_u8(dst, self.unknown1)?;
        if version >= 211 {
            write_bool(dst, self.flag_a)?;
            write_bool(dst, self.flag_b)?;
            write_bool(dst, self.flag_c)?;
        }
        if version <= 213 {
            write_u8(dst, self.unknown2)?;
        }
        write_u8(dst, self.unknown3)?;
        write_u8(dst, self.picking_enabled)?;
        write_u8(dst, self.collision_enabled)?;
        write_u16(dst, self.insert_type)?;
        write_u8(dst, self.locked)?;
        if version <= 212 {
            write_u8(dst, self.unknown4)?;
        }
        write_bool(dst, self.flag_d)?;
        if version <= 212 {
            write_u8(dst, self.unknown5)?;
        }
        self.changed_timestamp.save(dst)?;
        if version <= 212 {
            write_u8(dst, self.unknown6)?;
            write_u8(dst, self.unknown7)?;
        }
        if version >= 212 {
            write_bool(dst, self.flag_e)?;
        }
        if version >= 216 {
            write_u8(dst, self.is_savegame_relevant)?;
        }

        let accessor_count = self.accessors.len();
        write_u32(dst, accessor_count as u32)?;
        for accessor in &self.accessors {
            accessor.save(dst)?;
        }
        Ok(())
    }
}
