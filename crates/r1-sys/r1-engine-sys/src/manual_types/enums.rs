#[repr(u32)]
pub enum eCGfxShared_eECubeMapFace {
    PositiveX = 0x00000000,
    NegativeX = 0x00000001,
    PositiveY = 0x00000002,
    NegativeY = 0x00000003,
    PositiveZ = 0x00000004,
    NegativeZ = 0x00000005,
    None = 0x00000006,
}

#[repr(transparent)]
pub struct eEDepthMapFormat(u32);

#[repr(transparent)]
pub struct eCGfxShared_eECmpFunc(u32);

#[repr(transparent)]
pub struct eEShaderMaterialSpecialPassType(u32);

#[repr(transparent)]
pub struct eETransformSpace(u32);

#[repr(transparent)]
pub struct eEShaderMaterialLightmapCombi(u32);

#[repr(transparent)]
pub struct eEReflectType(u32);

#[repr(transparent)]
pub struct eAnimShared_eEMotionType(u32);

#[repr(transparent)]
pub struct eAnimShared_eEMotionPlayMode(u32);
