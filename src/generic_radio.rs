//! V5 Smart Radio

#[cfg(any(target_env = "v5", target_env = "exp"))]
use core::ffi::{c_char, c_int};

#[cfg(any(target_env = "v5", target_env = "exp"))]
use crate::{map_jump_table, V5_DeviceT};

#[cfg(any(target_env = "v5", target_env = "exp"))]
map_jump_table! {
    0xaa4 => pub fn vexDeviceGenericRadioConnection(device: V5_DeviceT, link_id: *mut c_char, r#type: c_int, ov: bool),
    0xaac => pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32,
    0xab0 => pub fn vexDeviceGenericRadioTransmit(device: V5_DeviceT, data: *const u8, size: u16) -> i32,
    0xabc => pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> u32,
    0xac0 => pub fn vexDeviceGenericRadioReceive(device: V5_DeviceT, data: *mut u8, size: u16) -> i32,
    0xac8 => pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool,
}
