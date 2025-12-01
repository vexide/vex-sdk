//! VEXos System Functions

use core::ffi::{VaList, c_char, c_void};

pub use vex_sdk::{
    EX_SIG_MAGIC, V5_SIG_MAGIC, V5_SIG_OPTIONS_EXIT, V5_SIG_OPTIONS_INDG, V5_SIG_OPTIONS_NONE,
    V5_SIG_OPTIONS_THDG, V5_SIG_OWNER_PARTNER, V5_SIG_OWNER_SYS, V5_SIG_OWNER_VEX,
    V5_SIG_TYPE_USER, date, time, vcodesig,
};

#[unsafe(no_mangle)]
pub extern "C" fn vexPrivateApiDisable(sig: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexStdlibMismatchError(installed_version: u32, required_version: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexScratchMemoryLock() -> bool {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexScratchMemoryUnock() {}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemTimeGet() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexGettime(pTime: *mut time) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexGetdate(pDate: *mut date) {
    unsafe {
        *pDate = date {
            da_year: 2016,
            da_day: 16,
            da_mon: 11,
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemMemoryDump() {}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemDigitalIO(pin: u32, value: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemStartupOptions() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemExitRequest() {}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemHighResTimeGet() -> u64 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemPowerupTimeGet() -> u64 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemLinkAddrGet() -> u32 {
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemTimerGet(timer: u32) -> u32 {
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemUsbStatus() -> u32 {
    0x3
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemTimerStop() {}

#[unsafe(no_mangle)]
pub extern "C" fn vexSystemTimerClearInterrupt() {}

#[unsafe(no_mangle)]
pub extern "C" fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexSystemWatchdogReinitRtos() -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemWatchdogGet() -> u32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexSystemUndefinedException() {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemFIQInterrupt() {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemIRQInterrupt() {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemSWInterrupt() {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemDataAbortInterrupt() {
    loop {}
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSystemPrefetchAbortInterrupt() {
    loop {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_vprintf(format: *const c_char, args: VaList) -> i32 {
    -1
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_vsprintf(
    out: *mut c_char,
    format: *const c_char,
    args: VaList,
) -> i32 {
    -1
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_vsnprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    args: VaList,
) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_printf(format: *const c_char, mut args: ...) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_sprintf(
    out: *mut c_char,
    format: *const c_char,
    mut args: ...
) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_snprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    mut args: ...
) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexSystemVersion() -> u32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexStdlibVersion() -> u32 {
    Default::default()
}
