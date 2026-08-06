//! Filesystem Access

use core::ffi::c_char;

/// See <http://elm-chan.org/fsw/ff/doc/rc.html> for details.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct FRESULT(pub core::ffi::c_uchar);

impl FRESULT {
    /// The function succeeded.
    pub const FR_OK: Self = Self(0);
    /// The lower layer reported that an unrecoverable hard error occured.
    pub const FR_DISK_ERR: Self = Self(1);
    /// Assertion failed and an insanity is detected in the internal process.
    pub const FR_INT_ERR: Self = Self(2);
    /// The lower layer reported that the storage device could not be got ready to work.
    pub const FR_NOT_READY: Self = Self(3);
    /// Could not find the file in the directory.
    pub const FR_NO_FILE: Self = Self(4);
    /// Could not follow the path. A sub-directory in the path name could not be found.
    pub const FR_NO_PATH: Self = Self(5);
    /// The given string is invalid as a path name.
    pub const FR_INVALID_NAME: Self = Self(6);
    /// The required access was denied.
    pub const FR_DENIED: Self = Self(7);
    /// Name collision. A file or sub-directory with the same name is already existing in the directory.
    pub const FR_EXIST: Self = Self(8);
    /// The file/directory object is invalid or the pointer is null.
    pub const FR_INVALID_OBJECT: Self = Self(9);
    /// A write mode operation against the write-protected medium.
    pub const FR_WRITE_PROTECTED: Self = Self(10);
    /// Invalid drive number is specified in the path name or a null pointer is given as the path name.
    pub const FR_INVALID_DRIVE: Self = Self(11);
    /// Work area for the logical drive has not been registered.
    pub const FR_NOT_ENABLED: Self = Self(12);
    /// Valid FAT volume could not be found in the drive.
    pub const FR_NO_FILESYSTEM: Self = Self(13);
    /// The `f_mkfs` function aborted before start in format.
    pub const FR_MKFS_ABORTED: Self = Self(14);
    /// The function was canceled due to a timeout of thread-safe control.
    pub const FR_TIMEOUT: Self = Self(15);
    /// The operation to the file or sub-directory was rejected by file sharing control.
    pub const FR_LOCKED: Self = Self(16);
    /// Not enough memory for the operation.
    pub const FR_NOT_ENOUGH_CORE: Self = Self(17);
    /// Number of open objects has been reached maximum value and no more object can be opened.
    pub const FR_TOO_MANY_OPEN_FILES: Self = Self(18);
    /// The given parameter is invalid or there is an inconsistent for the volume.
    pub const FR_INVALID_PARAMETER: Self = Self(19);
}
pub type FIL = core::ffi::c_void;

unsafe extern "system" {
    /// Mounts the SD card if not mounted, and returns the result.
    pub fn vexFileMountSD() -> FRESULT;
    /// Writes a newline seperated string of filenames in a given directory to `buffer`, with a max length of `len` characters.
    pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT;
    /// Opens a file in read-only mode. The mode paraemter is ignored.
    pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL;
    /// Opens a file in write and append mode.
    pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL;
    /// Opens or creates a file in write and truncate mode.
    pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL;
    /// Closes the given file descriptor.
    pub fn vexFileClose(fdp: *mut FIL);
    /// Writes `size * nItems` bytes from buf into the provided file descriptor.
    pub fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    /// Returns the size of the given file, in bytes.
    pub fn vexFileSize(fdp: *mut FIL) -> i32;
    /// Seeks the given file descriptor `offset` bytes forward. The `whence` parameter determines the refernce point for the seek offset.
    ///
    /// Note that VEXos does not support negative seek offsets.
    /// # `whence` values:
    /// - `0`/`SEEK_SET`: The file offset is set to the beginning of the file, plus `offset` bytes.
    /// - `1`/`SEEK_CUR`: The file offset is set to its current position, plus `offset` bytes.
    /// - `2`/`SEEK_END`: The file offset is set to the end of the file, plus `offset` bytes.
    pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT;
    /// Reads `size * nItems` bytes from the file descriptor into the provided buffer.
    pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    /// Returns whether or not the SD card is inserted. `drive` must be set to 0.
    pub fn vexFileDriveStatus(drive: u32) -> bool;
    /// Returns the current position of the file descriptor within the file.
    pub fn vexFileTell(fdp: *mut FIL) -> i32;
    /// Flushes the given file descriptor to disk.
    pub fn vexFileSync(fdp: *mut FIL);
    /// Returns the status of the file: if the file exists, a nonzero value is returned, else `0` is returned.
    pub fn vexFileStatus(filename: *const c_char) -> u32;
}
