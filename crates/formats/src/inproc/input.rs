use std::env::consts;

use crate::types::properties::enums::ActionKey;

use super::modules::ModuleAdmin;

/// gSAction
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ActionEvent {
    pub key: ActionKey,
    pub input_type: i32,
    pub event_duration: f32,
    pub stength: f32,
}
const _: () = assert!(size_of::<ActionEvent>() == 0x10);

#[repr(C)]
pub struct NativeInput(());

pub struct ActionAdmin {
    native: *const NativeActionAdmin,
}

#[repr(C)]
pub struct NativeActionAdmin(());

mod imports {
    use super::*;

    #[link(name = "Script")]
    unsafe extern "C" {
        #[link_name = "\x01?GetCurrentActionEvent@Input@@QEBAAEBUgSAction@@XZ"]
        pub(super) unsafe fn input_get_current_action_event(
            input: *const NativeInput,
        ) -> *const ActionEvent;

        #[link_name = "\x01input"]
        pub(super) unsafe static INPUT: *const NativeInput;
    }

    #[link(name = "Game")]
    unsafe extern "C" {
        #[link_name = "\x01?GetKeyPressDuration@gCActionAdmin@@QEBAMW4gEActionKey@@@Z"]
        pub(super) unsafe fn action_get_key_press_duration(
            this: *const NativeActionAdmin,
            key: ActionKey,
        ) -> f32;
        #[link_name = "\x01?SetCameraSens@gCActionAdmin@@QEAAXM@Z"]
        pub(super) unsafe fn action_set_camera_sens(
            this: *const NativeActionAdmin,
            sensitivity: f32,
        );
    }
}

pub fn get_current_action_event() -> *const ActionEvent {
    unsafe { imports::input_get_current_action_event(imports::INPUT) }
}

pub fn get_action_admin() -> ActionAdmin {
    let ptr = ModuleAdmin::get_instance()
        .find_module_by_str("gCActionAdmin")
        .unwrap();
    ActionAdmin { native: ptr as _ }
}

impl ActionAdmin {
    pub fn get_key_press_duration(&self, key: ActionKey) -> f32 {
        unsafe { imports::action_get_key_press_duration(self.native, key) }
    }

    pub fn set_camera_sens(&self, sensitivity: f32) {
        unsafe {
            imports::action_set_camera_sens(self.native, sensitivity);
        }
    }
}
