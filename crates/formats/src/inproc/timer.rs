/// eCTimer
pub struct EngineTimer {
    native: *const NativeEngineTimer,
}

#[repr(C)]
struct NativeEngineTimer {
    _opaque: [u8; 0],
}

mod imports {

    use super::*;

    #[link(name = "Engine")]
    unsafe extern "C" {
        #[link_name = "\x01?GetInstance@eCTimer@@SAAEAV1@XZ"]
        pub(super) unsafe fn timer_get_instance() -> *const NativeEngineTimer;
        #[link_name = "\x01?GetFrameTimeInSeconds@eCTimer@@QEBAMXZ"]
        pub(super) unsafe fn timer_get_frametime_in_s(this: *const NativeEngineTimer) -> f32;
    }
}

impl EngineTimer {
    pub fn get_instance() -> Self {
        unsafe {
            let native = imports::timer_get_instance();
            if native.is_null() {
                panic!("Failed to get eCTimer");
            }
            Self { native }
        }
    }

    pub fn get_frame_time_in_s(&self) -> f32 {
        unsafe { imports::timer_get_frametime_in_s(self.native) }
    }
}
