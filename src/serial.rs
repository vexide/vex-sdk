//! USB Serial Communication

use core::ffi::c_char;

use crate::map_jump_table;

map_jump_table! {
    0x898 => pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32,
    0x89c => pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32,
    0x8a0 => pub fn vexSerialReadChar(channel: u32) -> i32,
    0x8a4 => pub fn vexSerialPeekChar(channel: u32) -> i32,
    0x8ac => pub fn vexSerialWriteFree(channel: u32) -> i32,
}

pub unsafe extern "C" fn vex_printf(fmt: *const c_char, args: ...) -> i32 {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x0f0) as *const extern "C" fn(*const c_char, ...) -> i32))(fmt, args)
    }
}

pub unsafe extern "C" fn vexDebug(fmt: *const c_char, args: ...) -> i32 {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x0f0) as *const extern "C" fn(*const c_char, ...) -> i32))(fmt, args)
    }
}

pub unsafe extern "C" fn vex_sprintf(out: *mut c_char, fmt: *const c_char, args: ...) -> i32 {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x0f4) as *const extern "C" fn(*mut c_char, *const c_char, ...) -> i32))(out, fmt, args)
    }
}

pub unsafe extern "C" fn vex_snprintf(out: *mut c_char, max_len: u32, fmt: *const c_char, args: ...) -> i32 {
    unsafe {
        (*((crate::JUMP_TABLE_START + 0x0f8) as *const extern "C" fn(*mut c_char, u32, *const c_char, ...) -> i32))(out, max_len, fmt, args)
    }
}