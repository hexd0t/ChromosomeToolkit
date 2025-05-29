use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
    str,
};

/// bCString
#[derive(Clone)]
#[repr(C)]
pub struct bCString {
    //points to Null-Terminated str managed by engine
    data: *mut u8,
}

impl bCString {
    pub fn get_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.data as _).to_str().unwrap() }
    }
}
