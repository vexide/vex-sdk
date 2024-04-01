//! V5 Workcell Electromagnet

use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceMagnetDuration(core::ffi::c_uint);

impl V5_DeviceMagnetDuration {
    pub const kMagnetDurationShort: core::ffi::c_uint = 0;
    pub const kMagnetDurationMedium: core::ffi::c_uint = 1;
    pub const kMagnetDurationLong: core::ffi::c_uint = 2;
    pub const kMagnetDurationExtraLong: core::ffi::c_uint = 3;
}

map_jump_table! {
    0x578 => pub fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32),
    0x57c => pub fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32,
    0x580 => pub fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration),
    0x584 => pub fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration),
    0x588 => pub fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double,
    0x58c => pub fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double,
    0x590 => pub fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32,
}
