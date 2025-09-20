//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.

pub use vex_sdk::V5_DeviceLedColor;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
