use core::ffi::{c_char, c_void};

use crate::map_jump_table;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FRESULT {
    FR_OK = 0,
    FR_DISK_ERR,
    FR_INT_ERR,
    FR_NOT_READY,
    FR_NO_FILE,
    FR_NO_PATH,
    FR_INVALID_NAME,
    FR_DENIED,
    FR_EXIST,
    FR_INVALID_OBJECT,
    FR_WRITE_PROTECTED,
    FR_INVALID_DRIVE,
    FR_NOT_ENABLED,
    FR_NO_FILESYSTEM,
    FR_MKFS_ABORTED,
    FR_TIMEOUT,
    FR_LOCKED,
    FR_NOT_ENOUGH_CORE,
    FR_TOO_MANY_OPEN_FILES,
    FR_INVALID_PARAMETER,
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
