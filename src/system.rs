//! VEXos System Functions

use core::ffi::c_void;

use crate::map_jump_table;

/// Code Signature
///
/// The first 16 bytes of a user code binary should contain the user code
/// signature.  For simple user code programs this will be created by the
/// startup code in the runtime library, certain types of user code,
/// for example a virtual machine, may override the default settings to cause
/// the V5 system code to enable custom functionality yet TBD.
#[repr(C, packed)]
pub struct vcodesig {
    /// Magic, must be 'VXV5' 0x35565856 le
    pub magic: u32,

    /// Program type
    pub r#type: u32,

    /// Program originator
    pub owner: u32,

    pub pad: [u32; 2],

    /// Program options
    pub options: u32,
}

pub const V5_SIG_MAGIC: u32 = 0x35585658;
pub const V5_SIG_TYPE_USER: u32 = 0;
pub const V5_SIG_OWNER_SYS: u32 = 0;
pub const V5_SIG_OWNER_VEX: u32 = 1;
pub const V5_SIG_OWNER_PARTNER: u32 = 2;
pub const V5_SIG_OPTIONS_NONE: u32 = 0;
pub const V5_SIG_OPTIONS_INDG: u32 = (1 << 0);
pub const V5_SIG_OPTIONS_EXIT: u32 = (1 << 1);
pub const V5_SIG_OPTIONS_THDG: u32 = (1 << 2);

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
