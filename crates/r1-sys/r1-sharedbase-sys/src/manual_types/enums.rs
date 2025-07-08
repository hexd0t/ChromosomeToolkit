#[repr(u32)]
pub enum bEErrorType {
    Exception,
    Fatal,
    Fault,
}

#[repr(transparent)]
pub struct bEErrorPositionHandling(u32);

#[repr(u32)]
pub enum bEFileCreationMode {
    OpenExisting,
    OpenAlways,
    CreateAlways,
}

#[repr(u32)]
pub enum bEStreamSeekMode {
    Begin,
    Current,
    End,
}

#[repr(u32)]
pub enum bEResult {
    Err,
    Ok,
}

pub use bEResult as bEDoNotOverwriteMethod;

#[repr(transparent)]
pub struct bEMessageTypes(u32);
pub use bEMessageTypes as bEMessageFilterTypes;

#[repr(transparent)]
pub struct bEMessageCallbackPriority(u32);

#[repr(transparent)]
/// See https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority for valid values, usually +/- 15, 2, 1 or 0
pub struct bCThread_bEThreadPriority(i32);

#[repr(u32)]
pub enum bEMemoryAllocationType {
    Malloc,
    Calloc,
    New,
    NewArray,
    Realloc,
    Delete,
    DeleteArray,
    Free,
    Max,
}

#[repr(u32)]
pub enum bECheckPtr {
    Valid,
    Invalid,
    Corrupted,
    Disabled,
    Null,
}

#[repr(transparent)]
pub struct bCBox_bEIntersection(u32);

#[repr(u32)]
pub enum bCEngineDialogs_bEGE3DialogsLogMessageType {
    Error = 1,
    ErrorHistory,
    ErrorMemory,
    ErrorCallstack,
    FileNotUnique,
}

#[repr(u32)]
pub enum bCXMLParser_EEncoding {
    Default = 1,
}
