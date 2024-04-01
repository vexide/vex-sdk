//! V5 LED
//! 
//! This device is not sold by VEX and only exists as development hardware.

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceLedColor(core::ffi::c_uint);

impl V5_DeviceLedColor {
    pub const kLedColorBlack: core::ffi::c_uint = 0;
    pub const kLedColorRed: core::ffi::c_uint = 0xFF0000;
    pub const kLedColorGreen: core::ffi::c_uint = 0x00FF00;
    pub const kLedColorBlue: core::ffi::c_uint = 0x0000FF;
    pub const kLedColorYellow: core::ffi::c_uint = 0xFFFF00;
    pub const kLedColorCyan: core::ffi::c_uint = 0x00FFFF;
    pub const kLedColorMagenta: core::ffi::c_uint = 0xFF00FF;
    pub const kLedColorWhite: core::ffi::c_uint = 0xFFFFFF;
}

map_jump_table! {
    0x1e0 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor),
    0x1e4 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32),
    0x1e8 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor,
    0x1ec => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32,
}
