//! Filesystem Access

use core::ffi::{c_char, c_void};

use crate::map_jump_table;

pub mod FRESULT {
    pub type Type = core::ffi::c_uint;

    pub const FR_OK: Type = 0;
    pub const FR_DISK_ERR: Type = 1;
    pub const FR_INT_ERR: Type = 2;
    pub const FR_NOT_READY: Type = 3;
    pub const FR_NO_FILE: Type = 4;
    pub const FR_NO_PATH: Type = 5;
    pub const FR_INVALID_NAME: Type = 6;
    pub const FR_DENIED: Type = 7;
    pub const FR_EXIST: Type = 8;
    pub const FR_INVALID_OBJECT: Type = 9;
    pub const FR_WRITE_PROTECTED: Type = 10;
    pub const FR_INVALID_DRIVE: Type = 11;
    pub const FR_NOT_ENABLED: Type = 12;
    pub const FR_NO_FILESYSTEM: Type = 13;
    pub const FR_MKFS_ABORTED: Type = 14;
    pub const FR_TIMEOUT: Type = 15;
    pub const FR_LOCKED: Type = 16;
    pub const FR_NOT_ENOUGH_CORE: Type = 17;
    pub const FR_TOO_MANY_OPEN_FILES: Type = 18;
    pub const FR_INVALID_PARAMETER: Type = 19;
}
pub type FIL = c_void;

map_jump_table! {
    0x7d0 => pub fn vexFileMountSD() -> FRESULT::Type,
    0x7d4 => pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT::Type,
    0x7d8 => pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL,
    0x7dc => pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL,
    0x7e0 => pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL,
    0x7e4 => pub fn vexFileClose(fdp: *mut FIL),
    0x7ec => pub fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32,
    0x7f0 => pub fn vexFileSize(fdp: *mut FIL) -> i32,
    0x7f4 => pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT::Type,
    0x7f8 => pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32,
    0x7fc => pub fn vexFileDriveStatus(drive: u32) -> bool,
    0x800 => pub fn vexFileTell(fdp: *mut FIL) -> i32,
    0x804 => pub fn vexFileSync(fdp: *mut FIL),
    0x808 => pub fn vexFileStatus(filename: *const c_char) -> u32,
}
