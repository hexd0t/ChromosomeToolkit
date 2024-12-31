use bitflags::bitflags;
use serde::Deserialize;
use serde::Serialize;

use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use super::XmacChunkType;
use crate::archive::ArchiveReadTarget;
use crate::archive::ArchiveWriteTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::Vec3;
use crate::xmac::read_xmac_str;
use crate::xmac::write_xmac_str;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMorphTargets {
    pub targets: Vec<MorphTarget>,
    /// always 0 for R1
    pub unknown: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MorphTarget {
    pub name: String,

    pub range_min: f32,
    pub range_max: f32,

    pub mesh_deform_deltas: Vec<MeshDeformDeltas>,
    pub phoneme_set: PhonemeSet,

    /// always 0 for R1
    pub unknown1: u32,
}

bitflags! {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    pub struct PhonemeSet : u32 {
        // Original Phonemes:
        const None                                 = 0;
        const NeutralPose                          = 1 << 0; //1
        const M_B_P_X                              = 1 << 1; //2
        const AA_AO_OW                             = 1 << 2; //4
        const IH_AE_AH_EY_AY_H                     = 1 << 3; //8
        const AW                                   = 1 << 4; //10
        const N_NG_CH_J_DH_D_G_T_K_Z_ZH_TH_S_SH    = 1 << 5; //20
        const IY_EH_Y                              = 1 << 6; //40
        const UW_UH_OY                             = 1 << 7; //80
        const F_V                                  = 1 << 8; //100
        const L_EL                                 = 1 << 9; //200
        const W                                    = 1 << 10; //400
        const R_ER                                 = 1 << 11; //800
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeshDeformDeltas {
    pub node_id: XmacNodeId,

    /// vertex deltas
    /// are guaranteed to be sorted by vertex id
    pub deltas: Vec<MeshDeformDelta>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MeshDeformDelta {
    pub vertex_id: u32,
    pub position_delta: Vec3,
    pub normal_delta: Vec3,
    pub tangent_delta: Vec3,
}

impl XmacMorphTargets {
    pub fn load<R: ArchiveReadTarget>(
        src: &mut R,
        big_endian: bool,
        _multiply_order: bool,
        chunk_meta: &XmacChunkMeta,
    ) -> Result<Option<Self>> {
        println!("Loading MORPH TARGETS chunk...");
        match chunk_meta.version {
            1 => {
                let targets_count = read_u32_endian(src, big_endian)?;
                let unknown = read_u32_endian(src, big_endian)?;
                let mut targets = Vec::with_capacity(targets_count as usize);
                for _idx in 0..targets_count {
                    targets.push(MorphTarget::load(src, big_endian)?);
                }

                Ok(Some(Self { targets, unknown }))
            }
            ver => {
                println!(
                    "Unknown XMAC morph targets version {ver}@{:x}",
                    src.stream_position()?
                );
                Ok(None)
            }
        }
    }
    pub fn save<W: ArchiveWriteTarget>(
        &self,
        dst: &mut W,
        big_endian: bool,
    ) -> Result<XmacChunkMeta> {
        println!("Saving MORPH TARGETS chunk...");
        write_u32_endian(dst, self.targets.len() as u32, big_endian)?;
        write_u32_endian(dst, self.unknown, big_endian)?;
        let mut written = 4 + 4;
        for target in &self.targets {
            written += target.save(dst, big_endian)?;
        }

        Ok(XmacChunkMeta {
            type_id: XmacChunkType::StdPMorphTargets.into(),
            size: written as u32,
            version: 1,
        })
    }
}

impl MorphTarget {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let range_min = read_f32_endian(src, big_endian)?;
        let range_max = read_f32_endian(src, big_endian)?;
        // usually LOD, but those are missing in all other datastructures...
        let unknown1 = read_u32_endian(src, big_endian)?;
        let mesh_deform_deltas_count = read_u32_endian(src, big_endian)?;
        let transformations_count = read_u32_endian(src, big_endian)?;
        let phoneme_set = read_u32_endian(src, big_endian)?;
        let phoneme_set = PhonemeSet::from_bits(phoneme_set).unwrap();

        let name = read_xmac_str(src, big_endian)?;

        let mut mesh_deform_deltas = Vec::with_capacity(mesh_deform_deltas_count as usize);
        for _idx in 0..mesh_deform_deltas_count {
            mesh_deform_deltas.push(MeshDeformDeltas::load(src, big_endian)?);
        }
        assert_eq!(
            transformations_count, 0,
            "Transformations are not supported"
        );

        Ok(Self {
            range_min,
            range_max,
            name,
            mesh_deform_deltas,
            unknown1,
            phoneme_set,
        })
    }

    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        write_f32_endian(dst, self.range_min, big_endian)?;
        write_f32_endian(dst, self.range_max, big_endian)?;
        // usually LOD, but those are missing in all other datastructures...
        write_u32_endian(dst, self.unknown1, big_endian)?;
        write_u32_endian(dst, self.mesh_deform_deltas.len() as u32, big_endian)?;
        write_u32_endian(dst, 0, big_endian)?; //Mesh Transformations are not supported
        write_u32_endian(dst, self.phoneme_set.bits(), big_endian)?;
        let mut written = 4 + 4 + 4 + 4 + 4 + 4;
        written += write_xmac_str(dst, &self.name, big_endian)?;

        for delta in &self.mesh_deform_deltas {
            written += delta.save(dst, big_endian)?;
        }

        Ok(written)
    }
}

impl MeshDeformDeltas {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let node_id = XmacNodeId(read_u32_endian(src, big_endian)?);
        let min_val = read_f32_endian(src, big_endian)?;
        let max_val = read_f32_endian(src, big_endian)?;
        let val_span = max_val - min_val;
        let vertices_count = read_u32_endian(src, big_endian)? as usize;

        let mut deltas = vec![MeshDeformDelta::default(); vertices_count];
        // Position Vectors are compressed to 16 bit/component using the given min/max:
        for delta in deltas.iter_mut() {
            let x_comp = read_u16_endian(src, big_endian)?;
            let y_comp = read_u16_endian(src, big_endian)?;
            let z_comp = read_u16_endian(src, big_endian)?;
            let x = min_val + val_span * (x_comp as f32 / u16::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u16::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u16::MAX as f32);
            delta.position_delta = Vec3 { x, y, z };
        }
        // Normal Vectors are compressed to 8 bit/component in range -2.0/+2.0:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = -2.0 + 4.0 * (x_comp as f32 / u8::MAX as f32);
            let y = -2.0 + 4.0 * (y_comp as f32 / u8::MAX as f32);
            let z = -2.0 + 4.0 * (z_comp as f32 / u8::MAX as f32);
            delta.normal_delta = Vec3 { x, y, z };
        }
        // Tangent Vectors are compressed to 8 bit/component in range -2.0/+2.0:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = -2.0 + 4.0 * (x_comp as f32 / u8::MAX as f32);
            let y = -2.0 + 4.0 * (y_comp as f32 / u8::MAX as f32);
            let z = -2.0 + 4.0 * (z_comp as f32 / u8::MAX as f32);
            delta.tangent_delta = Vec3 { x, y, z };
        }
        for delta in deltas.iter_mut() {
            delta.vertex_id = read_u32_endian(src, big_endian)?;
        }
        deltas.sort_by_key(|d| d.vertex_id);
        Ok(Self { node_id, deltas })
    }
    pub fn save<W: ArchiveWriteTarget>(&self, dst: &mut W, big_endian: bool) -> Result<usize> {
        write_u32_endian(dst, self.node_id.0, big_endian)?;

        if self.deltas.is_empty() {
            print!("Empty morph?!?");
            return Ok(4);
        }

        let mut written = 4;

        let min_val = self
            .deltas
            .iter()
            .map(|d| d.position_delta.min_element())
            .min_by(|a, b| {
                a.partial_cmp(b)
                    .expect("Morph Delta must not contain NaNs!")
            })
            .unwrap();
        let max_val = self
            .deltas
            .iter()
            .map(|d| d.position_delta.max_element())
            .max_by(|a, b| {
                a.partial_cmp(b)
                    .expect("Morph Delta must not contain NaNs!")
            })
            .unwrap();

        write_f32_endian(dst, min_val, big_endian)?;
        write_f32_endian(dst, max_val, big_endian)?;
        write_u32_endian(dst, self.deltas.len() as u32, big_endian)?;
        written += 3 * 4;

        let val_span = max_val - min_val;

        let min_val = Vec3::new(min_val, min_val, min_val);
        let val_span = Vec3::new(val_span, val_span, val_span);
        let max_u16 = Vec3::new(u16::MAX as f32, u16::MAX as f32, u16::MAX as f32);
        for delta in &self.deltas {
            let comp = ((delta.position_delta - min_val) * max_u16 / val_span).round();
            write_u16_endian(dst, comp.x as u16, big_endian)?;
            write_u16_endian(dst, comp.y as u16, big_endian)?;
            write_u16_endian(dst, comp.z as u16, big_endian)?;
            written += 3 * 2;
        }

        let min_val = Vec3::new(-2.0, -2.0, -2.0);
        let val_span = Vec3::new(4.0, 4.0, 4.0);
        let max_u8 = Vec3::new(u8::MAX as f32, u8::MAX as f32, u8::MAX as f32);
        for delta in &self.deltas {
            let comp = ((delta.normal_delta - min_val) * max_u8 / val_span).round();
            write_u8(dst, comp.x as u8)?;
            write_u8(dst, comp.y as u8)?;
            write_u8(dst, comp.z as u8)?;
            written += 3 * 1;
        }
        for delta in &self.deltas {
            let comp = ((delta.tangent_delta - min_val) * max_u8 / val_span).round();
            write_u8(dst, comp.x as u8)?;
            write_u8(dst, comp.y as u8)?;
            write_u8(dst, comp.z as u8)?;
            written += 3 * 1;
        }

        for delta in &self.deltas {
            write_u32_endian(dst, delta.vertex_id, big_endian)?;
            written += 4;
        }

        Ok(written)
    }
}

impl Default for MeshDeformDelta {
    fn default() -> Self {
        Self {
            vertex_id: 0,
            position_delta: Vec3::ZERO,
            normal_delta: Vec3::ZERO,
            tangent_delta: Vec3::ZERO,
        }
    }
}
