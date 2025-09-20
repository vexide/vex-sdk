//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskAdd(
    callback: unsafe extern "C" fn() -> c_int,
    interval: c_int,
    label: *const c_char,
) {
}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void {
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskSleep(time: u32) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexTaskHardwareConcurrency() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBackgroundProcessing() {}

#[unsafe(no_mangle)]
pub extern "C" fn vexTasksRun() {}
