//! V5 Vision Sensor

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{
    V5VisionLedMode, V5VisionMode, V5VisionWBMode, V5VisionWifiMode, V5_DeviceVisionObject,
    V5_DeviceVisionRgb, V5_DeviceVisionSignature,
};

use crate::map_jump_table;

map_jump_table! {
    0x398 => pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode),
    0x39c => pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode,
    0x3a0 => pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32,
    0x3a4 => pub fn vexDeviceVisionObjectGet(device: V5_DeviceT, index: u32, object: *mut V5_DeviceVisionObject) -> i32,
    0x3a8 => pub fn vexDeviceVisionSignatureSet(device: V5_DeviceT, signature: *mut V5_DeviceVisionSignature),
    0x3ac => pub fn vexDeviceVisionSignatureGet(device: V5_DeviceT, id: u32, signature: *mut V5_DeviceVisionSignature) -> bool,
    0x3c0 => pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8),
    0x3c4 => pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8,
    0x3c8 => pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode),
    0x3cc => pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode,
    0x3c0 => pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb),
    0x3c4 => pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb,
    0x3c8 => pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode),
    0x3cc => pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode,
    0x3d0 => pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8),
    0x3d4 => pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8,
    0x3d8 => pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb),
    0x3dc => pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb,
    0x3e0 => pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode),
    0x3e4 => pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode,
}
