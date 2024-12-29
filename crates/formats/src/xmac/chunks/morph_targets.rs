use bitflags::bitflags;
use serde::Deserialize;
use serde::Serialize;

use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::Vec3;
use crate::xmac::read_xmac_str;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmacMorphTargets {
    targets: Vec<MorphTarget>,
    unknown: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MorphTarget {
    name: String,

    range_min: f32,
    range_max: f32,

    mesh_deform_deltas: Vec<MeshDeformDeltas>,
    phoneme_set: PhonemeSet,

    unknown1: u32,
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
    node_id: XmacNodeId,

    deltas: Vec<MeshDeformDelta>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MeshDeformDelta {
    vertex_id: u32,
    position_delta: Vec3,
    normal_delta: Vec3,
    tangent_delta: Vec3,
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
            "Transformations are not implemented yet"
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
}

impl MeshDeformDeltas {
    pub fn load<R: ArchiveReadTarget>(src: &mut R, big_endian: bool) -> Result<Self> {
        let node_id = XmacNodeId(read_u32_endian(src, big_endian)?);
        let min_val = read_f32_endian(src, big_endian)?;
        let max_val = read_f32_endian(src, big_endian)?;
        let val_span = max_val - min_val;
        let vertices_count = read_u32_endian(src, big_endian)? as usize;

        let mut deltas = vec![MeshDeformDelta::default(); vertices_count];
        // Position Vectors are compressed to 16 bit/component:
        for delta in deltas.iter_mut() {
            let x_comp = read_u16_endian(src, big_endian)?;
            let y_comp = read_u16_endian(src, big_endian)?;
            let z_comp = read_u16_endian(src, big_endian)?;
            let x = min_val + val_span * (x_comp as f32 / u16::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u16::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u16::MAX as f32);
            delta.position_delta = Vec3 { x, y, z };
        }
        // Normal Vectors are compressed to 8 bit/component:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = min_val + val_span * (x_comp as f32 / u8::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u8::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u8::MAX as f32);
            delta.normal_delta = Vec3 { x, y, z };
        }
        // Tangent Vectors are compressed to 8 bit/component:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = min_val + val_span * (x_comp as f32 / u8::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u8::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u8::MAX as f32);
            delta.tangent_delta = Vec3 { x, y, z };
        }
        for delta in deltas.iter_mut() {
            delta.vertex_id = read_u32_endian(src, big_endian)?;
        }
        Ok(Self { node_id, deltas })
    }
}

impl Default for MeshDeformDelta {
    fn default() -> Self {
        Self {
            vertex_id: 0,
            position_delta: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal_delta: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            tangent_delta: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
