use std::ops::Deref;
use std::sync::LazyLock;

use formats::inproc::input::get_action_admin;
use formats::inproc::script::ScriptInitData;
static SCRIPT_INIT_DATA: LazyLock<ScriptInitData> = LazyLock::new(ScriptInitData::default);

#[no_mangle]
pub unsafe extern "stdcall" fn ScriptInit() -> *const ScriptInitData {
    println!("Block-Fix ScriptInit");

    let action_admin = get_action_admin();
    action_admin.set_camera_sens(1.0);

    let script_init = SCRIPT_INIT_DATA.deref();

    script_init
}
