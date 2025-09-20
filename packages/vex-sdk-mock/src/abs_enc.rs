//! V5 Rotation Sensor

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReset(device: V5_DeviceT) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32) {}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool) {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32) {}
