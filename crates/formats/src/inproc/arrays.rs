use std::{
    ops::{Index, IndexMut},
    ptr::null_mut,
};

use super::string::NativeString;

/// bTObjArray
#[derive(Default)]
#[repr(C)]
pub struct ObjArray<T: Sized + Clone + 'static> {
    pub base: ArrayBase<T>,
}

unsafe impl<T: Sized + Clone + 'static> Send for ObjArray<T> {}
unsafe impl<T: Sized + Clone + 'static> Sync for ObjArray<T> {}
/// bTArrayBase
#[repr(C)]
pub struct ArrayBase<T: Sized> {
    pub data: *mut T,
    pub count: u32,
    pub capacity: u32,
}

impl<T: Sized + 'static + Clone> Default for ArrayBase<T> {
    fn default() -> Self {
        Self {
            data: null_mut(),
            count: 0,
            capacity: 0,
        }
    }
}

impl<T: Sized + 'static + Clone> Index<usize> for ArrayBase<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.count as usize {
            panic!("Out of range!");
        }
        unsafe { &(*self.data.add(index)) }
    }
}
impl<T: Sized + 'static + Clone> IndexMut<usize> for ArrayBase<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.count as usize {
            panic!("Out of range!");
        }
        unsafe { &mut (*self.data.add(index)) }
    }
}

/// bTStringObjMap
#[repr(C)]
pub struct StringObjMap<T: Sized + 'static + Clone>(pub(super) ValMap<NativeString, T>);

/// bTObjMap / bTValMap
#[repr(C)]
pub struct ValMap<K: Sized + 'static + Clone, T: Sized + 'static + Clone> {
    pub(super) vtable: *const (),
    pub(super) buckets: ObjArray<*mut ValMapNode<K, T>>,
    pub(super) count: u32,
    // pad 4
}
const _: () = assert!(size_of::<ValMap<String, String>>() == 0x20);

#[repr(C)]
#[derive(Clone)]
pub(super) struct ValMapNode<K: Sized + 'static + Clone, T: Sized + 'static + Clone> {
    pub(super) key: K,
    pub(super) val: T,
    pub(super) hash: u32,
    pub(super) next: *mut ValMapNode<K, T>,
}
