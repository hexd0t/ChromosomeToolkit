use serde::Deserialize;
use serde::Serialize;

use super::nodes::XmacNodeId;
use super::XmacChunkMeta;
use crate::archive::ArchiveReadTarget;
use crate::error::*;
use crate::helpers::*;
use crate::types::Vector3;
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

    unknown1: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeshDeformDeltas {
    node_id: XmacNodeId,

    deltas: Vec<MeshDeformDelta>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MeshDeformDelta {
    vertex_id: u32,
    position_delta: Vector3,
    normal_delta: Vector3,
    tangent_delta: Vector3,
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
        assert_eq!(
            transformations_count, 0,
            "Transformations are not implemented yet"
        );
        let phoneme_sets_count = read_u32_endian(src, big_endian)?;
        assert_eq!(
            phoneme_sets_count, 0,
            "Phoneme Sets are not implemented yet"
        );
        let name = read_xmac_str(src, big_endian)?;

        let mut mesh_deform_deltas = Vec::with_capacity(mesh_deform_deltas_count as usize);
        for _idx in 0..mesh_deform_deltas_count {
            mesh_deform_deltas.push(MeshDeformDeltas::load(src, big_endian)?);
        }

        Ok(Self {
            range_min,
            range_max,
            name,
            mesh_deform_deltas,
            unknown1,
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
            delta.position_delta = Vector3 { x, y, z };
        }
        // Normal Vectors are compressed to 8 bit/component:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = min_val + val_span * (x_comp as f32 / u8::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u8::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u8::MAX as f32);
            delta.normal_delta = Vector3 { x, y, z };
        }
        // Tangent Vectors are compressed to 8 bit/component:
        for delta in deltas.iter_mut() {
            let x_comp = read_u8(src)?;
            let y_comp = read_u8(src)?;
            let z_comp = read_u8(src)?;
            let x = min_val + val_span * (x_comp as f32 / u8::MAX as f32);
            let y = min_val + val_span * (y_comp as f32 / u8::MAX as f32);
            let z = min_val + val_span * (z_comp as f32 / u8::MAX as f32);
            delta.tangent_delta = Vector3 { x, y, z };
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
            position_delta: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal_delta: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            tangent_delta: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
