use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
    str,
};

/// bCString
pub enum EngineString {
    Owned(Box<NativeString>),
    Ref(*mut NativeString),
}
/// bCString
#[derive(Clone)]
#[repr(C)]
pub struct NativeString {
    //points to Null-Terminated str managed by engine
    data: *mut u8,
}

mod imports {
    use super::*;

    #[link(name = "SharedBase")]
    unsafe extern "C" {
        #[link_name = "\x01??0bCString@@QEAA@PEBD@Z"]
        pub(super) unsafe fn string_new_cstr(
            this: *mut NativeString,
            src: *const u8,
        ) -> *mut NativeString;
        #[link_name = "\x01??1bCString@@QEAA@XZ"]
        pub(super) unsafe fn string_drop(this: *mut NativeString);
    }
}

impl EngineString {
    pub fn new(content: &str) -> Self {
        let mut native_str = Box::new(NativeString { data: null_mut() });
        let content_nullterm = CString::new(content).unwrap();
        unsafe {
            imports::string_new_cstr(native_str.as_mut(), content_nullterm.as_bytes().as_ptr());
        }
        Self::Owned(native_str)
    }

    pub fn get_native_ptr(&self) -> *const NativeString {
        match self {
            EngineString::Owned(str) => str.as_ref(),
            EngineString::Ref(ptr) => *ptr,
        }
    }
    pub fn get_native_ptr_mut(&mut self) -> *mut NativeString {
        match self {
            EngineString::Owned(str) => str.as_mut(),
            EngineString::Ref(ptr) => *ptr,
        }
    }
}

impl Drop for EngineString {
    fn drop(&mut self) {
        let native_str = self.get_native_ptr_mut();
        unsafe {
            imports::string_drop(native_str);
        }
    }
}

impl NativeString {
    pub fn get_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.data as _).to_str().unwrap() }
    }
}
