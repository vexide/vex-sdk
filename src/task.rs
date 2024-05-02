//! VEXos Task Scheduler Functions

use crate::map_jump_table;

use core::ffi::{c_char, c_int, c_void};

map_jump_table! {
    0x028 =>
        /// Creates an RTOS task
        ///
        /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17>
        pub fn vexTaskAdd(
            callback: unsafe extern "C" fn() -> c_int,
            interval: c_int,
            label: *const c_char
        ),
    0x084 =>
        /// Gets a tasks's callback function and internal ID.
        ///
        /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L41>
        pub fn vexTaskGetCallbackAndId(
            index: u32,
            callback_id: *mut c_int,
        ) -> *mut c_void,
    0x06c =>
        /// Sets the current task to sleep for the specified amount of time (in milliseconds).
        ///
        /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18>
        pub fn vexTaskSleep(time: u32),
    0x140 =>
        /// Gets the number of concurrent threads supported by the hardware.
        ///
        /// Inferred from <https://api.vexcode.cloud/v5/search/static%20int32_t%20vex::thread::hardware_concurrency()/vex::thread/function>
        pub fn vexTaskHardwareConcurrency() -> i32,
    0xf74 => pub fn vexBackgroundProcessing(),
    0x05c =>
        /// VEXos User Background Processing Procedure
        ///
        /// This function handles transactions between the user processor and master
        /// processor when writing data to devices. This function should ideally be ran
        /// every 2mS for devices to be properly communicated with.
        pub fn vexTasksRun(),
}
