use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OpticalRaw {
    pub clear: u16,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OpticalRgb {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub brightness: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OpticalGesture {
    pub udata: u8,
    pub ddata: u8,
    pub ldata: u8,
    pub rdata: u8,
    pub gesture_type: u8,
    pub padding: u8,
    pub count: u16,
    pub time: u32,
}

map_jump_table! {
    0x528 => pub fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> core::ffi::c_double,
    0x52c => pub fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> core::ffi::c_double,
    0x530 => pub fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> core::ffi::c_double,
    0x534 => pub fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32,
    0x538 => pub fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut OpticalRgb),
    0x53c => pub fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32),
    0x540 => pub fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32,
    0x544 => pub fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32,
    0x548 => pub fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut OpticalRaw),
    0x550 => pub fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32),
    0x554 => pub fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32,
    0x558 => pub fn vexDeviceOpticalGestureGet(device: V5_DeviceT, pData: *mut OpticalGesture),
    0x55c => pub fn vexDeviceOpticalGestureEnable(device: V5_DeviceT),
    0x560 => pub fn vexDeviceOpticalGestureDisable(device: V5_DeviceT),
    0x564 => pub fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32),
    0xb40 => pub fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: core::ffi::c_double),
    0xb44 => pub fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> core::ffi::c_double,
}
