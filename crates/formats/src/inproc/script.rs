use std::thread;
use std::time::Duration;

use super::arrays::{ObjArray, StringObjMap};
use super::string::{EngineString, NativeString};

/// gSScriptInit
#[derive(Default)]
#[repr(C)]
pub struct ScriptInitData {
    pub script_ai_states: ObjArray<[u8; 0]>,
    pub script_ai_functions: ObjArray<[u8; 0]>,
    pub script_ai_callbacks: ObjArray<[u8; 0]>,
    pub scripts: ObjArray<[u8; 0]>,
}

/// eCScriptFunctionMap
pub struct ScriptFunctionMap {
    native: *const NativeScriptFunctionMap,
}
/// eSScriptBase
#[derive(Debug)]
pub struct ScriptBase {
    native: *mut NativeScriptBase,
}

/// eCScriptFunctionMap
#[repr(C)]
pub struct NativeScriptFunctionMap {
    ai_states: StringObjMap<NativeScriptBase>,
    ai_functions: StringObjMap<NativeScriptBase>,
    ai_callbacks: StringObjMap<NativeScriptBase>,
    scripts: StringObjMap<NativeScriptBase>,
}

/// eSScriptBase
#[repr(C)]
#[derive(Clone)]
pub struct NativeScriptBase {
    instance_counter: u32,
    function_map_node: usize,
    source: NativeString,
    script_dll: usize,
    function: *const (),
}

mod imports {

    use super::*;

    #[link(name = "Engine")]
    unsafe extern "C" {
        #[link_name = "\x01?GetInstance@eCScriptFunctionMap@@SAAEAV1@XZ"]
        pub(super) unsafe fn script_function_map_get_instance() -> *const NativeScriptFunctionMap;
        #[link_name = "\x01?GetScript@eCScriptFunctionMap@@QEBAPEBUeSScriptBase@@AEBVbCString@@@Z"]
        pub(super) unsafe fn script_function_map_get_script(
            this: *const NativeScriptFunctionMap,
            name: *const NativeString,
        ) -> *mut NativeScriptBase; //return should be const, but some mods need to break this
    }
}

impl ScriptFunctionMap {
    pub fn get_instance() -> Self {
        unsafe {
            let native = imports::script_function_map_get_instance();
            if native.is_null() {
                panic!("Failed to get eCScriptFunctionMap");
            }
            Self { native }
        }
    }

    pub fn get_script_names(&self) -> Vec<&str> {
        let mut result;
        unsafe {
            let scripts = &(*self.native).scripts.0;
            let ai_callbacks = &(*self.native).ai_callbacks.0;
            let ai_functions = &(*self.native).ai_functions.0;
            let ai_states = &(*self.native).ai_states.0;
            println!(
                "{} {} {} {}",
                scripts.count, ai_callbacks.count, ai_functions.count, ai_states.count
            );
            result = Vec::with_capacity(
                (scripts.count + ai_callbacks.count + ai_functions.count + ai_states.count)
                    as usize,
            );
            let lists = [
                &scripts.buckets.base,
                &ai_callbacks.buckets.base,
                &ai_functions.buckets.base,
                &ai_states.buckets.base,
            ];
            for buckets in lists {
                for bucket in 0..buckets.count {
                    let mut node = buckets[bucket as usize];
                    while !node.is_null() {
                        result.push((*node).key.get_str());
                        node = (*node).next;
                    }
                }
            }
        }
        result
    }

    pub fn get_script(&self, name: &str) -> Option<ScriptBase> {
        unsafe {
            let native_name = EngineString::new(name);
            let native =
                imports::script_function_map_get_script(self.native, native_name.get_native_ptr());
            if native.is_null() {
                None
            } else {
                Some(ScriptBase { native })
            }
        }
    }
}

impl ScriptBase {
    pub fn get_function(&self) -> *const () {
        unsafe { (*self.native).function }
    }
    pub fn set_function(&mut self, new: *const ()) {
        unsafe {
            (*self.native).function = new;
        }
    }
}
