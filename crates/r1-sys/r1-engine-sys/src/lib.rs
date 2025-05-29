#![allow(non_camel_case_types, non_snake_case, unused_imports)]

use r1_sharedbase_sys::*;

mod manual_types;

pub use manual_types::*;

include!(concat!(env!("OUT_DIR"), "/symbols.rs"));
