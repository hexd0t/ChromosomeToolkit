use gltf::json::scene::UnitQuaternion as GltfQuaternion;

use super::{Quaternion, Vector3};

impl From<GltfQuaternion> for Quaternion {
    fn from(value: GltfQuaternion) -> Self {
        Self {
            x: value.0[0],
            y: value.0[1],
            z: value.0[2],
            w: value.0[3],
        }
    }
}
impl From<Quaternion> for GltfQuaternion {
    fn from(value: Quaternion) -> Self {
        Self([value.x, value.y, value.z, value.w])
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}
impl From<Vector3> for [f32; 3] {
    fn from(value: Vector3) -> Self {
        [value.x, value.y, value.z]
    }
}
