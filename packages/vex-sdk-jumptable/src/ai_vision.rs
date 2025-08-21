//! V5 AI Vision Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceAiVisionCode, V5_DeviceAiVisionColor, V5_DeviceAiVisionObject};

use crate::map_jump_table;

map_jump_table! {
    0xcd4 => pub fn vexDeviceAiVisionClassNameGet(device: V5_DeviceT, id: i32, pName: *mut u8) -> i32,
    0xcc4 => pub fn vexDeviceAiVisionCodeGet(device: V5_DeviceT, id: u32, pCode: *mut V5_DeviceAiVisionCode) -> bool,
    0xcc0 => pub fn vexDeviceAiVisionCodeSet(device: V5_DeviceT, pCode: *mut V5_DeviceAiVisionCode),
    0xcbc => pub fn vexDeviceAiVisionColorGet(device: V5_DeviceT, id: u32, pColor: *mut V5_DeviceAiVisionColor) -> bool,
    0xcb8 => pub fn vexDeviceAiVisionColorSet(device: V5_DeviceT, pColor: *mut V5_DeviceAiVisionColor),
    0xcac => pub fn vexDeviceAiVisionModeGet(device: V5_DeviceT) -> u32,
    0xca8 => pub fn vexDeviceAiVisionModeSet(device: V5_DeviceT, mode: u32),
    0xcb0 => pub fn vexDeviceAiVisionObjectCountGet(device: V5_DeviceT) -> i32,
    0xcb4 => pub fn vexDeviceAiVisionObjectGet(device: V5_DeviceT, indexObj: u32, pObject: *mut V5_DeviceAiVisionObject) -> i32,
    0xcd8 => pub fn vexDeviceAiVisionSensorSet(device: V5_DeviceT, brightness: c_double, contrast: c_double),
    0xcc8 => pub fn vexDeviceAiVisionStatusGet(device: V5_DeviceT) -> u32,
    0xccc => pub fn vexDeviceAiVisionTemperatureGet(device: V5_DeviceT) -> c_double,
}
