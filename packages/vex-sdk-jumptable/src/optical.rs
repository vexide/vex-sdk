//! V5 Optical Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceOpticalGesture, V5_DeviceOpticalRaw, V5_DeviceOpticalRgb};

use crate::map_jump_table;

map_jump_table! {
    0x528 => pub fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double,
    0x52c => pub fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double,
    0x530 => pub fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double,
    0x534 => pub fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32,
    0x538 => pub fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb),
    0x53c => pub fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32),
    0x540 => pub fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32,
    0x544 => pub fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32,
    0x548 => pub fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw),
    0x550 => pub fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32),
    0x554 => pub fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32,
    0x558 => pub fn vexDeviceOpticalGestureGet(device: V5_DeviceT, pData: *mut V5_DeviceOpticalGesture) -> u32,
    0x55c => pub fn vexDeviceOpticalGestureEnable(device: V5_DeviceT),
    0x560 => pub fn vexDeviceOpticalGestureDisable(device: V5_DeviceT),
    0x564 => pub fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32),
    0xb40 => pub fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double),
    0xb44 => pub fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double,
}
