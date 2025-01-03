use super::string::EngineString;

pub struct ModuleAdmin {
    native: *const NativeModuleAdmin,
}

struct NativeModuleAdmin(());

mod imports {
    use crate::inproc::string::NativeString;

    use super::*;

    #[link(name = "Engine")]
    unsafe extern "C" {
        #[link_name = "\x01?GetInstance@eCModuleAdmin@@SAAEAV1@XZ"]
        pub(super) unsafe fn get_module_admin_instance() -> *const NativeModuleAdmin;

        #[link_name = "\x01?FindModule@eCModuleAdmin@@QEBAPEAVeCEngineComponentBase@@AEBVbCString@@@Z"]
        pub(super) unsafe fn module_admin_find_module_str(
            this: *const NativeModuleAdmin,
            name: *const NativeString,
        ) -> *const ();
    }
}

impl ModuleAdmin {
    pub fn get_instance() -> Self {
        Self {
            native: unsafe { imports::get_module_admin_instance() },
        }
    }

    pub fn find_module_by_str(&self, name: &str) -> Option<*const ()> {
        //ToDo: do the casting here via generics
        let name = EngineString::new(name);
        let result =
            unsafe { imports::module_admin_find_module_str(self.native, name.get_native_ptr()) };
        if result.is_null() {
            None
        } else {
            Some(result)
        }
    }
}
