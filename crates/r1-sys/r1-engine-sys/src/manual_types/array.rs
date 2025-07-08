use std::marker::PhantomData;

#[repr(C)]
pub struct eTVertexBufferArray<T: 'static> {
    _opaque: [u8; 0],
    _phantom: PhantomData<T>,
}
