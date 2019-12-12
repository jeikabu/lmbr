#![allow(non_snake_case)]

use lmbr_sys::root::{AZ, AssetBuilderSDK};

#[no_mangle]
pub fn IsAssetBuilder() -> i32 {
    0
}

#[no_mangle]
pub fn InitializeModule(shared_environment: AZ::EnvironmentInstance) {
    unsafe {
        AZ::Environment::Attach(shared_environment, false);
    }
    // Implement custom init
    lmbr_logger::init().unwrap();
    log::warn!("Hello AssetBuilder");
}

#[no_mangle]
pub fn UninitializeModule() {
    // Implement custom uninit

    unsafe {
        AZ::Environment::Detach();
    }
}

#[no_mangle]
pub fn ModuleRegisterDescriptors() {
    // Implement
}

#[no_mangle]
pub fn ModuleAddComponents(entity: *mut AZ::Entity) {
    // Implement

    unsafe {
        // From Code\Tools\AssetProcessor\Builders\LuaBuilder\Source\LuaBuilderWorker.h
        let uuid = "{166A7962-A3E4-4451-AC1A-AAD32E29C52C}";
        let uuid = AZ::Uuid_CreateString(uuid.as_ptr() as *const _, uuid.len());
        AssetBuilderSDK::BuilderLog(uuid, std::ffi::CString::new("Hello AssetBuilder").unwrap().as_ptr())
    }
}