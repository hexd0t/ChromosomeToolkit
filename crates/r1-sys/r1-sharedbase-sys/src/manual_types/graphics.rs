use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[repr(u32)]
pub enum bCColorBase_bEColor {
    /// #000000
    Black,
    /// #FFFFFF
    White,
    /// #C0C0C0
    Gray2,
    /// #AFAFAF
    Gray3,
    /// #7F7F7F
    Gray6,
    /// #4F4F4F
    Gray7,
    /// #D3D3D3
    Gray1,
    /// #808A87
    Gray4,
    /// #808069
    Gray5,
    /// #FF0000
    Red,
    /// #AF0000
    Red2,
    /// #7F0000
    Red3,
    /// #4F0000
    Red4,
    /// #00FF00
    Green,
    /// #00AF00
    Green2,
    /// #007F00
    Green3,
    /// #004F00
    Green4,
    /// #0000FF
    Blue,
    /// #0000AF
    Blue2,
    // #00007F
    Blue3,
    /// #00004F
    Blue4,
    /// #00FFFF
    Cyan,
    /// #00AFAF
    Cyan2,
    /// #007F7F
    Cyan3,
    /// #004F4F
    Cyan4,
    /// #FFFF00
    Yellow,
    /// #AFAF00
    Yellow2,
    /// #7F7F00
    Yellow3,
    /// #4F4F00
    Yellow4,
    /// #FF00FF
    Fuchsia,
    /// #AF00AF
    Fuchsia2,
    /// #7F007F
    Fuchsia3,
    /// #4F004F
    Fuchsia4,
    /// #292421
    Gray8,
    /// #FF8000
    Orange,
    /// #ED9121
    Orange2,
    /// #3B5E2B
    Green5,
    /// #082E54
    Blue5,
    /// #87CEEB
    Blue6,
    /// #4682B4
    Blue7,
    /// #7FFFD4
    Cyan5,
    /// #FFD700
    Yellow5,
    /// #DA70D6
    Fuchsia5,
    /// #A020F0
    Fuchsia6,
    /// #8F5E99
    Fuchsia7,
    /// #802A2A
    Brown,
    /// #87421F
    Brown2,
    /// #733D1A
    Brown3,
    /// #A39480
    Gray9,
    /// #FF7D40
    Orange3,
    /// #F0E68C
    Yellow6,
}

#[repr(u32)]
pub enum bEPixelFormat {
    Unknown = 0,
    RGB555 = 1,
    RGB565,
    RGBA4444,
    ARGB1555,
    RGBA8888,
    ARGB8888,
    RGB888,
}

bitflags! {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    #[repr(C)]
    pub struct bCUnitedRay_bERayExtension: u32 {
        const Invalid = 0x0;
        const Forward = 0x1;
        const Backward = 0x2;
        const Both = 0x3;
    }
}

bitflags! {
    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    #[repr(C)]
    pub struct bCTriangle_bESides: u32 {
        const Invalid = 0x0;
        const Front = 0x1;
        const Back = 0x2;
        const Both = 0x3;
    }
}
