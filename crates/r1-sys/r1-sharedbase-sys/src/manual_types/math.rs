#[repr(u32)]
pub enum bCVector_bECoordinate {
    X,
    Y,
    Z,
}

#[repr(C)]
pub struct bCVector4 {
    X: f32,
    Y: f32,
    Z: f32,
    W: f32,
}

#[repr(C)]
pub struct bCQuaternion {
    X: f32,
    Y: f32,
    Z: f32,
    W: f32,
}

#[repr(C)]
pub struct bCVector {
    X: f32,
    Y: f32,
    Z: f32,
}

#[repr(C)]
pub struct bCVector2 {
    X: f32,
    Y: f32,
}
const _: () = assert!(size_of::<bCVector2>() == 8);

#[repr(C)]
pub struct bCPoint {
    X: f32,
    Y: f32,
}
const _: () = assert!(size_of::<bCPoint>() == 8);

#[repr(C)]
pub struct bCRect {
    top_left: bCPoint,     //+0-7
    bottom_right: bCPoint, //+8-15
}
const _: () = assert!(size_of::<bCRect>() == 16);

#[repr(C)]
pub struct bCMotion {
    pub translation: bCVector,
    pub rotation: bCQuaternion,
}
const _: () = assert!(size_of::<bCRect>() == 16);

#[repr(u32)]
pub enum bCPerlinNoise_bEAlgorithm {
    Version1,
    Version2,
}

#[repr(u32)]
pub enum bCPeacheyNoise_bEAlgorithm {
    Version1,
    Version2,
}

#[repr(u32)]
pub enum bENoiseTurbulence {
    Normal,
    Absolute,
}

#[repr(u32)]
pub enum bENoiseAmplitude {
    Normal,
    Inverted,
}
