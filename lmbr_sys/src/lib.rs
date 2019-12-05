#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

extern "C" {
    pub fn ModuleInitISystem(
        pSystem: *mut std::ffi::c_void,
        moduleName: *const ::std::os::raw::c_char,
    );
    pub fn TickRequestBus_BroadcastResult_GetTickDeltaTime(results: *mut ::std::os::raw::c_float);
    pub fn TickRequestBus_BroadcastResult_GetTimeAtCurrentTick(results: *mut root::AZ::ScriptTimePoint);
}

use std::fmt;
use std::marker::PhantomData;
use root::AZStd;

// function_buffer is union so can't #[derive(Debug)]
impl fmt::Debug for AZStd::Internal::function_util::function_buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AZStd::Internal::function_util::function_buffer {{ }}")
    }
}

impl AZStd::chrono::system_clock {
    pub fn now() -> AZStd::chrono::system_clock_time_point {
        unsafe {
            let time = AZStd::GetTimeNowMicroSecond();
            let duration = AZStd::chrono::duration::from_duration_rep(time);
            AZStd::chrono::time_point::from_time_point_duration(duration)
        }
    }
}

impl<Rep> AZStd::chrono::duration<Rep> {
    pub fn from_duration_rep(val: Rep) -> Self {
        Self { m_rep: val, _phantom_0: PhantomData }
    }
}

impl<Duration> AZStd::chrono::time_point<Duration> {
    pub fn from_time_point_duration(val: Duration) -> Self {
        Self { m_d: val, _phantom_0: PhantomData }
    }
}