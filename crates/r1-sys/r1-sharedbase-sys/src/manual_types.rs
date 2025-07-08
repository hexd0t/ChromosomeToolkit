#![allow(non_camel_case_types, unused_imports)]

mod array;
mod enums;
mod graphics;
mod math;
mod memory;
mod properties;
mod string;
mod time;
mod variant;

pub use array::*;
pub use enums::*;
pub use graphics::*;
pub use math::*;
pub use memory::*;
pub use properties::*;
pub use string::*;
pub use time::*;
pub use variant::*;

pub use windows::Win32::Foundation::FILETIME as tagUI_FILETIME;
pub use windows::Win32::Foundation::POINT as tagPOINT;
pub use windows::Win32::Foundation::RECT as tagRECT;
pub use windows::Win32::Foundation::SYSTEMTIME as _SYSTEMTIME;
pub use windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS as _EXCEPTION_POINTERS;
// _GUID comes from the Windows headers, but is ABI-compatible with the UUID crate, which makes handling easier:
pub use uuid::Uuid as _GUID;
