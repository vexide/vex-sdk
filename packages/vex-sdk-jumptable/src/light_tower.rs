//! CTE Workcell Signal Tower

use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0x5b8 => pub fn vexDeviceLightTowerBlinkSet(device: V5_DeviceT, select: u8, mask: u8, onTime: i32, offTime: i32),
    0x5a4 => pub fn vexDeviceLightTowerColorSet(device: V5_DeviceT, color_id: u32, value: u32),
    0x5a8 => pub fn vexDeviceLightTowerRgbGet(device: V5_DeviceT) -> u32,
    0x5a0 => pub fn vexDeviceLightTowerRgbSet(device: V5_DeviceT, rgb_value: u32, xyw_value: u32),
    0x5b0 => pub fn vexDeviceLightTowerStatusGet(device: V5_DeviceT) -> u32,
    0x5b4 => pub fn vexDeviceLightTowerDebugGet(device: V5_DeviceT, id: i32) -> u32,
    0x5ac => pub fn vexDeviceLightTowerXywGet(device: V5_DeviceT) -> u32,
}
