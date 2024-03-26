//! Smart Port Generic Serial Communication

use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0xa50 => pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32),
    0xa54 => pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32),
    0xa58 => pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8),
    0xa5c => pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32,
    0xa60 => pub fn vexDeviceGenericSerialTransmit(device: V5_DeviceT, buffer: *mut u8, length: i32),
    0xa64 => pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32,
    0xa68 => pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32,
    0xa6c => pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32,
    0xa70 => pub fn vexDeviceGenericSerialReceive(device: V5_DeviceT, buffer: *mut u8, length: i32) -> i32,
    0xa74 => pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT),
}
