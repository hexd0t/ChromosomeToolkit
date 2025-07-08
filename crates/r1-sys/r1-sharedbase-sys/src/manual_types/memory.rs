use std::mem::offset_of;

use super::*;
use type_layout::TypeLayout;

#[repr(C)]
pub struct bCGCMemGeneration_bSGCRootDesc {
    buffer: *mut (),  //0-7
    generation: u16,  //8-9
    description: u16, //10-11
    buf_size: u16,    //12-13
    _unknown: u16,    //14-15
}

#[repr(C)]
pub struct bCGCMemGeneration {
    generation: u16,                            //+0-1
    roots: *mut bCGCMemGeneration_bSGCRootDesc, // +8-15
    _unknown1: u64,                             //16-23
    _unknown2: u64,                             //24-31
    /// self-referential pointer to beginning of data in this struct
    heap: *mut (), //+32-39
    /// self-referential pointer to next unused in data
    next: *mut (), //+40-47
    // Total Memory is hardcoded to 20000 bytes:
    data: [u8; 20000], //48...
}

const _: () = assert!(offset_of!(bCGCMemGeneration, data) == 48);
const _: () = assert!(size_of::<bCGCMemGeneration>() == 48 + 20000);

// Reference: SharedBase.dll:GetMemorySubSystemName
#[repr(u32)]
pub enum bEMemorySubSystem {
    None,
    Global,
    PropertySystem,
    FileSystem,
    Material,
    Animation,
    Mesh,
    Audio,
    Physics,
    Rendering,
    Scene,
    ResourceCache,
    Localization,
    Navigation,
    GUI,
    All,
}
