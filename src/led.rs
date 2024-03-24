use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_DeviceLedColor {
    kLedColorBlack = 0,
    kLedColorRed = 0xFF0000,
    kLedColorGreen = 0x00FF00,
    kLedColorBlue = 0x0000FF,
    kLedColorYellow = 0xFFFF00,
    kLedColorCyan = 0x00FFFF,
    kLedColorMagenta = 0xFF00FF,
    kLedColorWhite = 0xFFFFFF,
}

map_jump_table! {
    0x1e0 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor),
    0x1e4 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32),
    0x1e8 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor,
    0x1ec => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32,
}
