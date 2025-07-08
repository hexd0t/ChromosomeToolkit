use std::marker::PhantomData;

use super::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[repr(u32)]
pub enum bEPropertyTypeDetail {
    Unknown,
    /// bCString
    String,
    /// bCUnicodeString      
    UnicodeString,
    /// bCGuid
    GUID,
    /// bCVector
    Vector,
    /// bCVector4
    Vector4,
    /// bCMatrix
    Matrix,
    /// bCMatrix3
    Matrix3,
    /// bCBox
    Box,
    /// bCSphere
    Sphere,
    /// bCQuaternion
    Quaternion,
    /// bCObjectBase
    ObjectBase,
    /// struct derived from bTPropertyObject
    PropertyObject,
    /// f32
    Float,
    /// f64
    Double,
    /// bool
    Bool,
    /// i8
    Char,
    /// i64
    Int64,
    /// i32
    Long,
    /// i16
    Short,
    /// i32
    Int,
    /// Enum
    Enum,
}

#[repr(transparent)]
pub struct bCPropertyID_bSCore([u8; 16]);

#[repr(C)]
pub struct bCPropertyID {
    core: bCPropertyID_bSCore,
    count: u32,
}

#[repr(u32)]
pub enum bEPropertyType {
    Unknown = 0,
    Member = 1,
    Function = 2,
}

#[repr(C, packed)]
pub struct bCPropertyTypeBase {
    _vtable: *mut (),                          //+0-7
    name: bCString,                            //8-15,
    description: bCString,                     //16-23,
    category: bCString,                        //24-31
    property_type: bEPropertyType,             //32-35
    attributes: bCPropertyTypeBase_Attributes, //36
}

const _: () = assert!(size_of::<bCPropertyTypeBase>() == 37);

bitflags! {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    #[repr(C)]
    pub struct bCPropertyTypeBase_Attributes: u8 {
        const ReadOnly = 0x1;
        const Patchable = 0x2;
    }
}

#[repr(C)]
pub struct bTPropertyContainer<T: 'static> {
    _opaque: [u8; 0],
    _phantom: PhantomData<T>,
}
