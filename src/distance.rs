use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0x500 => pub fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32,
    0x504 => pub fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32,
    0x508 => pub fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32,
    0x518 => pub fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32,
    0x51c => pub fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double,
}
