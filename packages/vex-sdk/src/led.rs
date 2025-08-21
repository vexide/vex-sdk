//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.


use crate::{map_jump_table};
use vex_sdk_types::V5_DeviceT;

pub use vex_sdk_types::V5_DeviceLedColor;

map_jump_table! {
    0x1e0 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor),
    0x1e4 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32),
    0x1e8 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor,
    0x1ec => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32,
}
