//! V5 Distance Sensor

use core::ffi::c_double;

use crate::V5_DeviceT;

unsafe extern "system" {
    pub unsafe fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double;
}
