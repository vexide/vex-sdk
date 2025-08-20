//! V5 Rotation Sensor

use crate::{map_jump_table};
use vex_sdk_sys::V5_DeviceT;

map_jump_table! {
    0x488 => pub fn vexDeviceAbsEncReset(device: V5_DeviceT),
    0x48c => pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32),
    0x490 => pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32,
    0x494 => pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32,
    0x498 => pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32,
    0x49c => pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool),
    0x4a0 => pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool,
    0x4a4 => pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32,
    0x4c0 => pub fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32),
}
