use std::ffi::c_char;

#[repr(C)]
pub union bCVariant_bUType {
    u8: u8,
    i8: i8,
    u16: u16,
    i16: i16,
    u32: u32,
    i32: i32,
    u64: u64,
    i64: i64,
    bool: bool,
    f32: f32,
    f64: f64,
    void_ptr: *mut (),
    str_ptr: *mut c_char,
}

#[repr(u32)]
pub enum bCVariant_bEType {
    None,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    Int,
    U64,
    I64,
    Bool,
    F32,
    F64,
    VoidPtr,
    StrPtr,
}
