//! USB Serial Communication
unsafe extern "system" {
    /// Writes a single byte to the Brain's USB serial output buffer.
    ///
    /// Returns -1 on failure and a differing value on success.
    pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32;
    /// Writes the bytes in the buffer provided to the Brain's USB serial output buffer.
    ///
    /// Returns the number of btes written, or -1 on failure.
    pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32;
    /// Reads and returns a single byte from the Brain's USB serial input buffer, or -1 on failure.
    pub fn vexSerialReadChar(channel: u32) -> i32;
    /// Reads and returns a single byte from the Brain's USB serial input buffer without removing it, or -1 on failure.
    pub fn vexSerialPeekChar(channel: u32) -> i32;
    /// Returns the number of free bytes in the Brain's USB serial output buffer, or -1 on failure.
    pub fn vexSerialWriteFree(channel: u32) -> i32;
}
