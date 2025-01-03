use std::sync::LazyLock;
use std::{ops::Deref, sync::OnceLock};

use formats::inproc::input::{get_action_admin, get_current_action_event};
use formats::inproc::script::{ScriptFunctionMap, ScriptInitData};
use formats::inproc::timer::EngineTimer;
use formats::types::properties::enums::ActionKey;

static SCRIPT_INIT_DATA: LazyLock<ScriptInitData> = LazyLock::new(ScriptInitData::default);
type KeyPressedFn = fn(*const (), *const (), *const (), i32) -> i32;
static ORIG_KEY_PRESSED: OnceLock<KeyPressedFn> = OnceLock::new();

pub extern "stdcall" fn on_key_pressed(
    spu: *const (),
    self_entity: *const (),
    other_entity: *const (),
    args: i32,
) -> i32 {
    let key_event = get_current_action_event();
    let key = unsafe { *key_event };

    let action_admin = get_action_admin();
    action_admin.set_camera_sens(1.0);
    println!(
        "SecDur: {} / {key:?}",
        action_admin.get_key_press_duration(ActionKey::SecondaryAction)
    );

    ORIG_KEY_PRESSED.get().unwrap()(spu, self_entity, other_entity, args)
}

#[no_mangle]
pub unsafe extern "stdcall" fn ScriptInit() -> *const ScriptInitData {
    winapi::um::consoleapi::AllocConsole();
    println!("ScriptInit");

    let script_map = ScriptFunctionMap::get_instance();
    // let scripts = script_map.get_script_names();
    // let mut outf = BufWriter::new(File::create("scripts.txt").unwrap());
    // for script in scripts {
    //     writeln!(&mut outf, " {script}").unwrap();
    // }
    // outf.flush().unwrap();
    // drop(outf);
    // println!("Export done");
    let mut key_handler = script_map.get_script("OnMeleeHandleInput");

    if key_handler.is_none() || key_handler.as_ref().unwrap().get_function().is_null() {
        println!("OnMeleeHandleInput is not initialized yet!");
    } else {
        println!("detouring");
        ORIG_KEY_PRESSED
            .set(std::mem::transmute(
                key_handler.as_ref().unwrap().get_function(),
            ))
            .unwrap();
        key_handler
            .as_mut()
            .unwrap()
            .set_function(on_key_pressed as _);

        println!("detoured");
    }

    let script_init = SCRIPT_INIT_DATA.deref();

    script_init
}
