use super::*;
use std::ffi::c_char;

#[repr(C)]
pub struct eCGfxShared_eSGfxLayersDesc {
    pub width: u32,                   // 0000
    pub height: u32,                  // 0004
    pub depth: u32,                   // 0008
    pub cube_map: bool,               // 000C
    pub level_count: u32,             // 0010
    pub usage: u32,                   // 0014
    pub format: eEColorFormat,        // 0018
    pub pool: eEPoolType,             // 001C
    pub resource_size: u32,           // 0020
    pub function_name: *const c_char, // 0024
}

const _: () = assert!(size_of::<eCGfxShared_eSGfxLayersDesc>() == 0x28);
