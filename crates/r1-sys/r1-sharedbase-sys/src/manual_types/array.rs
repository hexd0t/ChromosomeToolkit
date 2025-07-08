use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::null_mut,
};
use type_layout::TypeLayout;

use super::*;

#[derive(Default)]
#[repr(C)]
pub struct bTValArray<T: 'static> {
    pub base: bTArrayBase<T>,
}

#[repr(C)]
pub struct bTObjArray<T: 'static>(pub bTValArray<T>);
const _: () = assert!(size_of::<bTObjArray<()>>() == 16);

#[repr(C)]
pub struct bTPtrArray<T: 'static>(pub bTValArray<T>);
const _: () = assert!(size_of::<bTPtrArray<()>>() == 16);

#[repr(C)]
pub struct bTArrayBase<T: 'static> {
    pub data: *mut T,  //+0-7
    pub count: u32,    //+8-11<
    pub capacity: u32, //+12-15
}

const _: () = assert!(size_of::<bTArrayBase<()>>() == 16);

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

#[repr(C)]
pub struct bTStringObjMap<T: 'static>(pub bTObjMap<bCString, T>);

#[repr(C)]
pub struct bTStringValMap<T: 'static>(pub bTValMap<bCString, T>);

#[repr(C)]
pub struct bTObjMap<K: 'static, T: 'static>(pub bTValMap<K, T>);

#[repr(C)]
pub struct bTValMap<K: 'static + Sized, T: 'static + Sized> {
    // not sure about the vtable, it doesn't seem to have virtual functions...?
    pub(super) vtable: *const (),
    pub(super) buckets: bTObjArray<*mut ValMapNode<K, T>>,
    pub(super) count: u32,
    // pad 4
}
const _: () = assert!(size_of::<bTValMap<String, String>>() == 0x20);

#[repr(C)]
pub struct ValMapNode<K: Sized + 'static, T: Sized + 'static> {
    pub(super) key: K,
    pub(super) val: T,
    pub(super) hash: u32,
    pub(super) next: *mut ValMapNode<K, T>,
}

#[repr(C)]
pub struct bTArrayMap_Entry<K: 'static + Sized, T: 'static + Sized> {
    key: K,
    value: T,
}

#[repr(C)]
pub struct bTArrayMap<K: 'static + Sized, T: 'static + Sized> {
    pub base: bTArrayBase<bTArrayMap_Entry<K, T>>,
}

#[repr(C)]
pub struct bTArrayBase_bCConstIterator<T: 'static> {
    _opaque: [u8; 0],
    _phantom: PhantomData<T>,
}
