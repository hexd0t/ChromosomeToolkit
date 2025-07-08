use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
    str,
};

#[derive(Clone)]
#[repr(C)]
pub struct bCString {
    ///points to Null-Terminated str managed by engine
    data: *mut u8,
}
const _: () = assert!(size_of::<bCString>() == 8);

impl bCString {
    pub fn get_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.data as _).to_str().unwrap() }
    }
}
