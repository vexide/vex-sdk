//! V5 Workcell Electromagnet

use core::ffi::c_double;
pub use vex_sdk_sys::V5_DeviceMagnetDuration;
use crate::map_jump_table;

use vex_sdk_sys::V5_DeviceT;

map_jump_table! {
    0x578 => pub fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32),
    0x57c => pub fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32,
    0x580 => pub fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration),
    0x584 => pub fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration),
    0x588 => pub fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double,
    0x58c => pub fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double,
    0x590 => pub fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32,
}
