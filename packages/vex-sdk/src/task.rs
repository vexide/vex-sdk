//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

unsafe extern "system" {
    /// Creates a cooperative VEXos task.
    ///
    /// The task can later be polled by calling [`vexTasksRun`] outside of a task.
    pub unsafe fn vexTaskAdd(
        callback: extern "system" fn() -> c_int,
        interval: c_int,
        label: *const c_char,
    );
    pub unsafe fn vexTaskGetCallbackAndId(index: u32, callback_id: *mut c_int) -> *mut c_void;
    /// Yields the current task for the specified amount of time.
    ///
    /// This function will either switch execution to another task or to the last caller of
    /// [`vexTasksRun`].
    ///
    /// # Safety
    ///
    /// Must be called from inside a task spawned by [`vexTaskAdd`] or an equivalent.
    pub unsafe fn vexTaskSleep(time: u32);
    /// No-op. In certain SDKs from VEX this function is an alias of [`vexTasksRun`].
    ///
    /// This function logs "Sdk Mismatch" to the system event log.
    pub safe fn vexBackgroundProcessing();
    /// Runs any pending VEXos tasks.
    ///
    /// These could be user-spawned tasks via [`vexTaskAdd`], or system tasks like
    pub safe fn vexTasksRun();
}
