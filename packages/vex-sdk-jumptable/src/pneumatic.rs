//! CTE Workcell Pneumatics Control

pub use vex_sdk::V5_DevicePneumaticCtrl;

use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0xc28 => pub fn vexDevicePneumaticActuationStatusGet(device: V5_DeviceT, ac1: *mut u16, ac2: *mut u16, ac3: *mut u16, ac4: *mut u16) -> u32,
    0xc08 => pub fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool),
    0xc10 => pub fn vexDevicePneumaticCtrlSet(device: V5_DeviceT, pCtrl: *mut V5_DevicePneumaticCtrl),
    0xc20 => pub fn vexDevicePneumaticCylinderPwmSet(device: V5_DeviceT, id: u32, bState: bool, pwm: u8),
    0xc0c => pub fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool),
    0xc1c => pub fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32,
    0xc18 => pub fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8),
    0xc14 => pub fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32,
}
