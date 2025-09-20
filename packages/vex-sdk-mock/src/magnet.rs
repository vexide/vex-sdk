//! V5 Workcell Electromagnet

use core::ffi::c_double;

pub use vex_sdk::V5_DeviceMagnetDuration;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
