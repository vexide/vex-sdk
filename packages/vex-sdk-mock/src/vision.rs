//! V5 Vision Sensor

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{
    V5_DeviceVisionObject, V5_DeviceVisionRgb, V5_DeviceVisionSignature, V5VisionLedMode,
    V5VisionMode, V5VisionWBMode, V5VisionWifiMode,
};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionObjectGet(
    device: V5_DeviceT,
    index: u32,
    object: *mut V5_DeviceVisionObject,
) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionSignatureSet(
    device: V5_DeviceT,
    signature: *mut V5_DeviceVisionSignature,
) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionSignatureGet(
    device: V5_DeviceT,
    id: u32,
    signature: *mut V5_DeviceVisionSignature,
) -> bool {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode {
    Default::default()
}
