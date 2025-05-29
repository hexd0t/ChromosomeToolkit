use std::{
    ops::{Index, IndexMut},
    ptr::null_mut,
};

use super::*;

#[derive(Default)]
#[repr(C)]
pub struct bTValArray<T: 'static> {
    pub base: bTArrayBase<T>,
}

#[repr(C)]
pub struct bTObjArray<T: 'static>(pub bTValArray<T>);

#[repr(C)]
pub struct bTPtrArray<T: 'static>(pub bTValArray<T>);

#[repr(C)]
pub struct bTArrayBase<T: 'static> {
    pub data: *mut T,
    pub count: u32,
    pub capacity: u32,
}

impl<T: 'static> Default for bTArrayBase<T> {
    fn default() -> Self {
        Self {
            data: null_mut(),
            count: 0,
            capacity: 0,
        }
    }
}

// To index into an array, we need to know the element size,
// hence these are only implemented if the stored type is Sized
impl<T: Sized + 'static> Index<usize> for bTArrayBase<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.count as usize {
            panic!("Out of range!");
        }
        unsafe { &(*self.data.add(index)) }
    }
}
impl<T: Sized + 'static> IndexMut<usize> for bTArrayBase<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.count as usize {
            panic!("Out of range!");
        }
        unsafe { &mut (*self.data.add(index)) }
    }
}

/// bTStringObjMap
#[repr(C)]
pub struct bTStringObjMap<T: 'static>(pub bTObjMap<bCString, T>);

/// bTStringObjMap
#[repr(C)]
pub struct bTStringValMap<T: 'static>(pub bTValMap<bCString, T>);

/// bTObjMap
#[repr(C)]
pub struct bTObjMap<K: 'static, T: 'static>(pub bTValMap<K, T>);

/// bTValMap
#[repr(C)]
pub struct bTValMap<K: 'static + Sized, T: 'static + Sized> {
    pub(super) vtable: *const (),
    pub(super) buckets: bTObjArray<*mut ValMapNode<K, T>>,
    pub(super) count: u32,
    // pad 4
}
const _: () = assert!(size_of::<bTValMap<String, String>>() == 0x20);

#[repr(C)]
pub(super) struct ValMapNode<K: Sized + 'static, T: Sized + 'static> {
    pub(super) key: K,
    pub(super) val: T,
    pub(super) hash: u32,
    pub(super) next: *mut ValMapNode<K, T>,
}
