//! USB Serial Communication

unsafe extern "system" {
    pub safe fn vexSerialWriteChar(channel: u32, c: u8) -> i32;
    pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32;
    pub safe fn vexSerialReadChar(channel: u32) -> i32;
    pub safe fn vexSerialPeekChar(channel: u32) -> i32;
    pub safe fn vexSerialWriteFree(channel: u32) -> i32;
}
