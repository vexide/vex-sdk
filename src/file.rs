//! Filesystem Access

use core::ffi::{c_char, c_void};

use crate::map_jump_table;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct FRESULT(pub core::ffi::c_uint);

impl FRESULT {
    pub const FR_OK: Self = Self(0);
    pub const FR_DISK_ERR: Self = Self(1);
    pub const FR_INT_ERR: Self = Self(2);
    pub const FR_NOT_READY: Self = Self(3);
    pub const FR_NO_FILE: Self = Self(4);
    pub const FR_NO_PATH: Self = Self(5);
    pub const FR_INVALID_NAME: Self = Self(6);
    pub const FR_DENIED: Self = Self(7);
    pub const FR_EXIST: Self = Self(8);
    pub const FR_INVALID_OBJECT: Self = Self(9);
    pub const FR_WRITE_PROTECTED: Self = Self(10);
    pub const FR_INVALID_DRIVE: Self = Self(11);
    pub const FR_NOT_ENABLED: Self = Self(12);
    pub const FR_NO_FILESYSTEM: Self = Self(13);
    pub const FR_MKFS_ABORTED: Self = Self(14);
    pub const FR_TIMEOUT: Self = Self(15);
    pub const FR_LOCKED: Self = Self(16);
    pub const FR_NOT_ENOUGH_CORE: Self = Self(17);
    pub const FR_TOO_MANY_OPEN_FILES: Self = Self(18);
    pub const FR_INVALID_PARAMETER: Self = Self(19);
}
pub type FIL = c_void;

map_jump_table! {
    0x7d0 => pub fn vexFileMountSD() -> FRESULT,
    0x7d4 => pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT,
    0x7d8 => pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL,
    0x7dc => pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL,
    0x7e0 => pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL,
    0x7e4 => pub fn vexFileClose(fdp: *mut FIL),
    0x7ec => pub fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32,
    0x7f0 => pub fn vexFileSize(fdp: *mut FIL) -> i32,
    0x7f4 => pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT,
    0x7f8 => pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32,
    0x7fc => pub fn vexFileDriveStatus(drive: u32) -> bool,
    0x800 => pub fn vexFileTell(fdp: *mut FIL) -> i32,
    0x804 => pub fn vexFileSync(fdp: *mut FIL),
    0x808 => pub fn vexFileStatus(filename: *const c_char) -> u32,
}
