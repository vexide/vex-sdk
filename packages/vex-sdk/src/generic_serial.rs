//! Smart Port Generic Serial Communication

use crate::V5_DeviceT;

unsafe extern "system" {
    /// Enables generic serial mode on the given port. Note that the provided port may take several milliseconds to be configured, even after this function returns.
    pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32);
    /// Sets the baudrate for the given port. The maximum allowed baud rate is 921600.
    pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32);
    /// Writes a single byte to the given port's FIFO output buffer.
    ///
    /// Returns -1 on error and a different value otherwise.
    pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32;
    /// Returns the number of bytes free in the port's FIFO output buffer, or -1 on error.
    pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32;
    /// Writes the bytes from the given buffer into the port's FIFO output buffer.
    ///
    /// Returns the number of bytes written, or -1 on error.
    pub fn vexDeviceGenericSerialTransmit(
        device: V5_DeviceT,
        buffer: *const u8,
        length: i32,
    ) -> i32;
    /// Reads and returns a single char from the FIFO input buffer, or -1 on error.
    pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32;
    /// Reads and return a single char from the FIFO input buffer without removing it, or -1 on error.
    pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32;
    /// Returns the number of bytes free in the port's FIFO input buffer, or -1 on error.
    pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32;
    /// Reads the bytes from the port's FIFO input buffer into the provided buffer.
    ///
    /// Returns the number of bytes read, or -1 on error.
    pub fn vexDeviceGenericSerialReceive(device: V5_DeviceT, buffer: *mut u8, length: i32) -> i32;
    /// Clears the internal input and output FIFO buffers. Despite the name, this function does *not* flush output buffers.
    pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT);
}
