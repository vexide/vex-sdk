//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;

/// # Safety
///
/// - `device` must be a valid, non-null pointer to a device handle
pub unsafe extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    9999
}

/// # Safety
///
/// - `device` must be a valid, non-null pointer to a device handle
pub unsafe extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

/// # Safety
///
/// - `device` must be a valid, non-null pointer to a device handle
pub unsafe extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}

/// # Safety
///
/// - `device` must be a valid, non-null pointer to a device handle
pub unsafe extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    -1
}

/// # Safety
///
/// - `device` must be a valid, non-null pointer to a device handle
pub unsafe extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    0.0
}
