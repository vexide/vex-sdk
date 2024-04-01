//! V5 LED
//! 
//! This device is not sold by VEX and only exists as development hardware.

use crate::{map_jump_table, V5_DeviceT};

pub mod V5_DeviceLedColor {
    pub type Type = core::ffi::c_uint;

    pub const kLedColorBlack: Type = 0;
    pub const kLedColorRed: Type = 0xFF0000;
    pub const kLedColorGreen: Type = 0x00FF00;
    pub const kLedColorBlue: Type = 0x0000FF;
    pub const kLedColorYellow: Type = 0xFFFF00;
    pub const kLedColorCyan: Type = 0x00FFFF;
    pub const kLedColorMagenta: Type = 0xFF00FF;
    pub const kLedColorWhite: Type = 0xFFFFFF;
}

map_jump_table! {
    0x1e0 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor::Type),
    0x1e4 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32),
    0x1e8 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor::Type,
    0x1ec => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32,
}
