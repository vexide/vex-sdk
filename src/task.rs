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
    0xf74 => pub fn vexBackgroundProcessing(),
    0x05c =>
        /// Ticks the CPU1 Scheduler
        ///
        /// This function is responsible for running VEXos tasks on CPU1. It must be called by
        /// the runtime ideally every 2mS to allow for internal OS tasks on the user processor
        /// to run. The scheduler is entirely cooperative, so the runtime must ensure that the CPU
        /// gets time to regularly execute these tasks.
        /// 
        /// VEXos has several background tasks in its scheduler responsible for handling transactions
        /// between CPU1 and CPU0, including tasks for handling device reads, serial flushing, USB,
        /// and other important operations that must occur in the background alongside user code.
        pub fn vexTasksRun(),
}
