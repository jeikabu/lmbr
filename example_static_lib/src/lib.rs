
use log::{info};

#[no_mangle]
pub extern fn example_static_lib() {
    lmbr_logger::init().unwrap();
    info!("RUST!!!!");
}