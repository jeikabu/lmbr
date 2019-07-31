
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::{ffi::CString, os::raw::c_char};

struct LmbrLogger;

static LOGGER: LmbrLogger = LmbrLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

impl log::Log for LmbrLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{}", record.args());
            log("RUST", &message);
        }
    }

    fn flush(&self) {}
}

pub fn log(window: &str, message: &str) {
    let window = CString::new(window).unwrap();
    let window = window.as_bytes_with_nul().as_ptr() as *const c_char;
    let message = CString::new(message).unwrap();
    let message = message.as_bytes_with_nul().as_ptr() as *const c_char;
    unsafe {
        lmbr_sys::root::AZ::Debug::Trace::Output(window, message);
    }
}