use crate::map_jump_table;

map_jump_table! {
    0x898 => pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32,
    0x89c => pub fn vexSerialWriteBuffer(channel: u32, data: *mut u8, data_len: u32) -> i32,
    0x8a0 => pub fn vexSerialReadChar(channel: u32) -> i32,
    0x8a4 => pub fn vexSerialPeekChar(channel: u32) -> i32,
    0x8ac => pub fn vexSerialWriteFree(channel: u32) -> i32,
}
