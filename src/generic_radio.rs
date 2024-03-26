//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0xaa4 => pub fn vexDeviceGenericRadioConnection(device: V5_DeviceT, link_id: *mut c_char, r#type: c_int, ov: bool),
    0xaac => pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32,
    0xab0 => pub fn vexDeviceGenericRadioTransmit(device: V5_DeviceT, data: *mut u8, size: u16),
    0xabc => pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> u32,
    0xac0 => pub fn vexDeviceGenericRadioReceive(device: V5_DeviceT, data: *mut u8, size: u16),
    0xac8 => pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool,
}
