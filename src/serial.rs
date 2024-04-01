//! USB Serial Communication

use core::ffi::{c_char, VaList};

use crate::map_jump_table;

map_jump_table! {
    0x898 => pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32,
    0x89c => pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32,
    0x8a0 => pub fn vexSerialReadChar(channel: u32) -> i32,
    0x8a4 => pub fn vexSerialPeekChar(channel: u32) -> i32,
    0x8ac => pub fn vexSerialWriteFree(channel: u32) -> i32,
    0x0f0 => pub fn vex_vprintf(format: *const c_char, args: VaList) -> i32,
    0x0f4 => pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList) -> i32,
    0x0f8 => pub fn vex_vsnprintf(out: *mut c_char, max_len: u32, format: *const c_char, args: VaList) -> i32,
}