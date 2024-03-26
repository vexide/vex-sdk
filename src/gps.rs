//! V5 GPS

use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsRaw {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsAttitude {
    pub pitch: c_double, // x
    pub roll: c_double,  // y
    pub yaw: c_double,   // z

    // spacial position on the field
    pub position_x: c_double,
    pub position_y: c_double,
    pub position_z: c_double,

    // alternative roll, pitch and yaw
    pub az: c_double,
    pub el: c_double,
    pub rot: c_double,
}

#[repr(C)]
#[repr(packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsQuaternion {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

map_jump_table! {
    0x5c8 => pub fn vexDeviceGpsReset(device: V5_DeviceT),
    0x5cc => pub fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double,
    0x5d0 => pub fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double,
    0x5d4 => pub fn vexDeviceGpsQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceGpsQuaternion),
    0x5d8 => pub fn vexDeviceGpsAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceGpsAttitude, bRaw: bool),
    0x5dc => pub fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw),
    0x5e0 => pub fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw),
    0x5e4 => pub fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32,
    0x5e8 => pub fn vexDeviceGpsTemperatureGet(device: V5_DeviceT) -> c_double,
    0x5f0 => pub fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32),
    0x5f4 => pub fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32,
    0x5f8 => pub fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32),
    0x5fc => pub fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double),
    0x600 => pub fn vexDeviceGpsOriginGet(device: V5_DeviceT, ox: *mut c_double, oy: *mut c_double),
    0x604 => pub fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double),
    0x608 => pub fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double,
    0x60c => pub fn vexDeviceGpsInitialPositionSet(device: V5_DeviceT, initial_x: c_double, initial_y: c_double, initial_rotation: c_double),
    0x614 => pub fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double,
}
