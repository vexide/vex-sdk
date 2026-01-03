//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

unsafe extern "system" {
    pub unsafe fn vexTaskAdd(
        callback: unsafe extern "system" fn() -> c_int,
        interval: c_int,
        label: *const c_char,
    );
    pub unsafe fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void;
    pub safe fn vexTaskSleep(time: u32);
    pub safe fn vexBackgroundProcessing();
    pub safe fn vexTasksRun();
}
