//! Smart Port Generic Serial Communication

use crate::V5_DeviceT;

unsafe extern "system" {
    pub unsafe fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32);
    pub unsafe fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32);
    pub unsafe fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32;
    pub unsafe fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceGenericSerialTransmit(
        device: V5_DeviceT,
        buffer: *const u8,
        length: i32,
    ) -> i32;
    pub unsafe fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceGenericSerialReceive(device: V5_DeviceT, buffer: *mut u8, length: i32) -> i32;
    pub unsafe fn vexDeviceGenericSerialFlush(device: V5_DeviceT);
}
