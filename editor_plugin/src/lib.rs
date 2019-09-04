
use log::info;
use std::os::raw::{c_char, c_void};

const MODULE_NAME: &[u8] = b"Rust Editor Plugin Example\0";
const GUID: &[u8] = b"{98C1DC36-5D1E-4CF6-91CE-AFA1117CE81F}\0";

#[no_mangle]
pub extern fn rust_editor_plugin_init(p_isystem: *mut c_void) {
    lmbr_logger::init().unwrap();
    unsafe {
        lmbr_sys::ModuleInitISystem(p_isystem, rust_editor_plugin_module_name());
    }
    info!("Initialized");
}

#[no_mangle]
pub extern fn rust_editor_plugin_module_name() -> *const c_char {
    MODULE_NAME.as_ptr() as *const _
}

#[no_mangle]
pub extern fn rust_editor_plugin_guid() -> *const c_char {
    GUID.as_ptr() as *const _
}

#[no_mangle]
pub extern fn rust_editor_plugin_version() -> u32 {
    1
}

#[no_mangle]
pub extern fn rust_editor_plugin_name() -> *const c_char {
    rust_editor_plugin_module_name() as *const _
}

#[no_mangle]
pub extern fn rust_editor_plugin_can_exit_now() -> bool {
    true
}

#[no_mangle]
pub extern fn rust_editor_plugin_release() {
}