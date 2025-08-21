//! VEXos System Functions

use core::ffi::{c_char, c_void, VaList};

pub use vex_sdk_sys::{
    date, time, vcodesig, EX_SIG_MAGIC, V5_SIG_MAGIC, V5_SIG_OPTIONS_EXIT, V5_SIG_OPTIONS_INDG,
    V5_SIG_OPTIONS_NONE, V5_SIG_OPTIONS_THDG, V5_SIG_OWNER_PARTNER, V5_SIG_OWNER_SYS,
    V5_SIG_OWNER_VEX, V5_SIG_TYPE_USER,
};

use crate::{map_jump_table, JUMP_TABLE_START};

map_jump_table! {
    0x01c =>
        /// special use only ! Talk to James.
        pub fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32,
    0x998 => pub fn vexScratchMemoryLock() -> bool,
    0x99c => pub fn vexScratchMemoryUnlock(),
    0x118 =>
        /// Returns the time since program start with millisecond precision.
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
        /// Returns the time since program start with microsecond precision.
        pub fn vexSystemHighResTimeGet() -> u64,
    0x138 =>
        /// Returns the time since power on with microsecond precision.
        pub fn vexSystemPowerupTimeGet() -> u64,
    0x13c =>
        /// Returns the address in memory of a linked file to the current user program, or `0`, if no file is linked.
        ///
        /// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
        /// similar
        pub fn vexSystemLinkAddrGet() -> u32,
    0x174 => pub fn vexSystemUsbStatus() -> u32,
    0x8c0 => pub fn vexSystemTimerStop(),
    0x8c4 => pub fn vexSystemTimerClearInterrupt(),
    0x8c8 => pub fn vexSystemTimerReinitForRtos(priority: u32, handler: extern "system" fn(data: *mut c_void)) -> i32,
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
    0x0f0 => pub fn vex_vprintf(format: *const c_char, args: VaList) -> i32,
    0x0f4 => pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList) -> i32,
    0x0f8 => pub fn vex_vsnprintf(out: *mut c_char, max_len: u32, format: *const c_char, args: VaList) -> i32,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_printf(format: *const c_char, mut args: ...) -> i32 {
    unsafe { vex_vprintf(format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_sprintf(
    out: *mut c_char,
    format: *const c_char,
    mut args: ...
) -> i32 {
    unsafe { vex_vsprintf(out, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_snprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    mut args: ...
) -> i32 {
    unsafe { vex_vsnprintf(out, max_len, format, args.as_va_list()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexSystemVersion() -> u32 {
    unsafe { core::ptr::read_volatile((JUMP_TABLE_START + 0x1000) as *const u32) }
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexStdlibVersion() -> u32 {
    unsafe { core::ptr::read_volatile((JUMP_TABLE_START + 0x1004) as *const u32) }
}
