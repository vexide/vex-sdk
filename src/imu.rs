use core::ffi::c_double;

use crate::{device::V5_DeviceT, map_jump_table};

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuRaw {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuQuaternion {
    pub a: c_double,
    pub b: c_double,
    pub c: c_double,
    pub d: c_double,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum V5ImuOrientationMode {
    kImuOrientationZUp = 0x00,
    kImuOrientationZDown = 0x10,
    kImuOrientationXUp = 0x20,
    kImuOrientationXDown = 0x30,
    kImuOrientationYUp = 0x40,
    kImuOrientationYDown = 0x50,
    #[default]
    kImuOrientationAuto = 0x80,
}

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuAttitude {
    pub pitch: c_double,
    pub roll: c_double,
    pub yaw: c_double,
}

map_jump_table! {
    0x410 => pub fn vexDeviceImuReset(device: V5_DeviceT),
    0x414 => pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double,
    0x418 => pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double,
    0x41c => pub fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion),
    0x420 => pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude),
    0x424 => pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw),
    0x428 => pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw),
    0x42c => pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32,
    0x438 => pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32),
    0x43c => pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32,
    0x444 => pub fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32),
}
