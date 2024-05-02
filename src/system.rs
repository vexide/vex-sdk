//! VEXos System Functions

use core::ffi::c_void;

use crate::map_jump_table;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct time {
    /// Hours
    pub ti_hour: u8,
    /// Minutes
    pub ti_min: u8,
    /// Seconds
    pub ti_sec: u8,
    /// Hundredths of seconds
    pub ti_hund: u8,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct date {
    /// Year - 1980
    pub da_year: u16,
    /// Day of the month
    pub da_day: u8,
    /// Month (1 = Jan)
    pub da_mon: u8,
}

map_jump_table! {
    0x10 => pub fn vexStdlibMismatchError(param_1: u32, param_2: u32),
    0x01c =>
        /// special use only ! Talk to James.
        pub fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32,
    0x998 => pub fn vexScratchMemoryLock() -> bool,
    0x99c => pub fn vexScratchMemoryUnock(),
    0x118 =>
        /// Gets the time since program start with millisecond precision.
        pub fn vexSystemTimeGet() -> u32,
    0x11c => pub fn vexGettime() -> time,
    0x120 => pub fn vexGetdate() -> date,
    0x124 => pub fn vexSystemMemoryDump(),
    0x128 => pub fn vexSystemDigitalIO(pin: u32, value: u32),
    0x12c => pub fn vexSystemStartupOptions() -> u32,
    0x130 =>
        /// Exits the current user program, returning to the main program screen.
        pub fn vexSystemExitRequest(),
    0x134 =>
        /// Gets the time since program start with microsecond precision.
        pub fn vexSystemHighResTimeGet() -> u64,
    0x138 =>
        /// Gets the time since power on with microsecond precision.
        pub fn vexSystemPowerupTimeGet() -> u64,
    0x13c =>
        /// Gets the address in memory of a linked file to the current user program, or `0`, if no file is linked.
        ///
        /// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
        /// similar
        pub fn vexSystemLinkAddrGet() -> u32,
    0x168 => pub fn vexSystemTimerGet(param_1: u32) -> u32,
    0x16c => pub fn vexSystemTimerEnable(param_1: u32) -> u32,
    0x170 => pub fn vexSystemTimerDisable(param_1: u32),
    0x174 => pub fn vexSystemUsbStatus() -> u32,
    0x8c0 => pub fn vexSystemTimerStop(),
    0x8c4 => pub fn vexSystemTimerClearInterrupt(),
    0x8c8 => pub fn vexSystemTimerReinitForRtos(priority: u32, handler: extern "C" fn(data: *mut c_void)) -> i32,
    0x8cc => pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32),
    0x8d0 => pub fn vexSystemWatchdogReinitRtos() -> i32,
    0x8d4 => pub fn vexSystemWatchdogGet() -> u32,
    0x910 => pub fn vexSystemBoot(),
    0x914 => pub fn vexSystemUndefinedException(),
    0x918 => pub fn vexSystemFIQInterrupt(),
    0x91c => pub fn vexSystemIQRQnterrupt(),
    0x920 => pub fn vexSystemSWInterrupt(),
    0x924 => pub fn vexSystemDataAbortInterrupt(),
    0x928 => pub fn vexSystemPrefetchAbortInterrupt(),
}
