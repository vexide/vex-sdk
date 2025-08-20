//! USB Serial Communication

use core::ffi::{c_char, VaList};

use crate::map_jump_table;

map_jump_table! {
    0x898 =>
        /// Writes a single byte to the serial FIFO output buffer.
        ///
        /// # Arguments
        ///
        /// - `channel`: The serial communications channel to write to. Use `1` for stdio.
        /// - `c`: The byte to write.
        ///
        /// # Return
        ///
        /// The number of bytes written, or -1 if an internal error occurred.
        pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32,
    0x89c =>
        /// Writes an arbitrary buffer to the serial FIFO output buffer, returning how
        /// many bytes were written. The output buffer has a maximum size of 2048 bytes,
        /// meaning that a larger buffer may be truncated.
        ///
        /// # Arguments
        ///
        /// - `channel`: The serial communications channel to write to. Use `1` for stdio.
        /// - `data`: A buffer of bytes to write.
        /// - `data_len`: The length of the specified buffer.
        ///
        /// # Return
        ///
        /// The number of bytes written, or -1 if an internal error occurred.
        pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32,
    0x8a0 =>
        /// Reads a single byte from the input buffer.
        ///
        /// # Arguments
        ///
        /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
        ///
        /// # Return
        ///
        /// The next byte in the input buffer, or -1 if no character is available to be read.
        pub fn vexSerialReadChar(channel: u32) -> i32,
    0x8a4 =>
        /// Returns the next available byte to be read in the input buffer without removing
        /// it from the buffer.
        ///
        /// # Arguments
        ///
        /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
        ///
        /// # Return
        ///
        /// The next byte in the input buffer, or -1 if no character is available to be read.
        pub fn vexSerialPeekChar(channel: u32) -> i32,
    0x8ac =>
        /// Returns the number of free bytes (out of `2048`) remaining in the serial output
        /// buffer.
        ///
        /// # Arguments
        ///
        /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
        ///
        /// # Return
        ///
        /// The number of remaining available bytes.
        pub fn vexSerialWriteFree(channel: u32) -> i32,
    0x0f0 => pub fn vex_vprintf(format: *const c_char, args: VaList) -> i32,
    0x0f4 => pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList) -> i32,
    0x0f8 => pub fn vex_vsnprintf(out: *mut c_char, max_len: u32, format: *const c_char, args: VaList) -> i32,
}

pub unsafe extern "C" fn vex_printf(format: *const c_char, mut args: ...) -> i32 {
    unsafe { vex_vprintf(format, args.as_va_list()) }
}

pub unsafe extern "C" fn vex_sprintf(
    out: *mut c_char,
    format: *const c_char,
    mut args: ...
) -> i32 {
    unsafe { vex_vsprintf(out, format, args.as_va_list()) }
}

pub unsafe extern "C" fn vex_snprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    mut args: ...
) -> i32 {
    unsafe { vex_vsnprintf(out, max_len, format, args.as_va_list()) }
}
