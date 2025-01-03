//! Derived from gotham-rs (MIT License, https://github.com/gotham-rs/gotham/)

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
    pin::Pin,
    sync::RwLock,
};

// With TypeIds as keys, there's no need to hash them. They are already hashes
// themselves, coming from the compiler. The IdHasher just holds the u64 of
// the TypeId, and then returns it, instead of doing any bit fiddling.
#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

static VTABLES: RwLock<Option<VTableStore>> = RwLock::new(None);

pub fn get_vtable<T: Any + Send + Sync, F: FnOnce() -> T>(initializer: F) -> *const T {
    let read = VTABLES.read().unwrap();
    if let Some(store) = &*read {
        if let Some(vtable) = store.get() {
            return vtable;
        }
    }
    drop(read);
    let mut write = VTABLES.write().unwrap();
    if write.is_none() {
        write.replace(VTableStore::new());
    }
    let store = write.as_mut().unwrap();
    store.put(initializer());
    store.get().unwrap()
}

pub struct VTableStore {
    data: HashMap<TypeId, Pin<Box<dyn Any + Send + Sync + 'static>>, BuildHasherDefault<IdHasher>>,
}

impl VTableStore {
    /// Creates a new, empty `State` container. This is for internal Gotham use, because the
    /// ability to create a new `State` container would allow for libraries and applications to
    /// incorrectly discard important internal data.
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }

    pub fn put<T: Send + Sync + 'static>(&mut self, t: T) {
        let type_id = TypeId::of::<T>();
        self.data.insert(type_id, Box::pin(t));
    }

    pub fn get<T>(&self) -> Option<*const T>
    where
        T: Send + 'static,
    {
        let type_id = TypeId::of::<T>();
        self.data
            .get(&type_id)
            .and_then(|b| b.downcast_ref::<T>())
            .map(|p| p as *const T)
    }
}
