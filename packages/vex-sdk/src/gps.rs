//! V5 GPS

use core::ffi::c_double;
use vex_sdk_sys::V5_DeviceT;
use crate::map_jump_table;

pub use vex_sdk_sys::{V5_DeviceGpsQuaternion, V5_DeviceGpsAttitude, V5_DeviceGpsRaw};

map_jump_table! {
    0x5c8 => pub fn vexDeviceGpsReset(device: V5_DeviceT),
    0x5cc => pub fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double,
    0x5d0 => pub fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double,
    0x5d4 => pub fn vexDeviceGpsQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceGpsQuaternion),
    0x5d8 => pub fn vexDeviceGpsAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceGpsAttitude, bRaw: bool),
    0x5dc => pub fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw),
    0x5e0 => pub fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw),
    0x5e4 => pub fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32,
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
