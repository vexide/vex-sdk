//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    9999
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    0.0
}
