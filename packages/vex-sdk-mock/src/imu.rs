//! V5 Inertial Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceImuAttitude, V5_DeviceImuQuaternion, V5_DeviceImuRaw};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuReset(device: V5_DeviceT) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32) {}
