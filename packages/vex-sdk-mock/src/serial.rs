//! USB Serial Communication

#[unsafe(no_mangle)]
pub extern "C" fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    Default::default()
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexSerialReadChar(channel: u32) -> i32 {
    -1
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSerialPeekChar(channel: u32) -> i32 {
    -1
}
#[unsafe(no_mangle)]
pub extern "C" fn vexSerialWriteFree(channel: u32) -> i32 {
    Default::default()
}
