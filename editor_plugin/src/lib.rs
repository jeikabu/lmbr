
use log::{error};
use std::{ffi::{c_void, CString}, os::raw::c_char};

#[no_mangle]
pub extern fn func(p_isystem: *mut c_void) {
    let window = CString::new("RUST").unwrap();
    let window = window.as_bytes_with_nul().as_ptr() as *const c_char;
    let message = CString::new("RUST PLUGIN!!!!").unwrap();
    let message = message.as_bytes_with_nul().as_ptr() as *const c_char;
    
    lmbr_logger::init().unwrap();
    error!("RUST PLUGIN BEGIN");
    
    unsafe {
        lmbr_sys::ModuleInitISystem(p_isystem, message);
        lmbr_sys::root::AZ::Debug::Trace::Output(window, message);
    }
    error!("RUST PLUGIN END");
}

#[no_mangle]
pub extern fn CreateRustPluginInstance() {

}