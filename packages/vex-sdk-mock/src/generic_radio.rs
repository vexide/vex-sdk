//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioConnection(
    device: V5_DeviceT,
    link_id: *mut c_char,
    r#type: c_int,
    ov: bool,
) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioTransmit(
    device: V5_DeviceT,
    data: *const u8,
    size: u16,
) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioReceive(
    device: V5_DeviceT,
    data: *mut u8,
    size: u16,
) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool {
    Default::default()
}
