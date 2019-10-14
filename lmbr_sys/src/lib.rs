#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

extern "C" {
    pub fn ModuleInitISystem(
        pSystem: *mut std::ffi::c_void,
        moduleName: *const ::std::os::raw::c_char,
    );
}