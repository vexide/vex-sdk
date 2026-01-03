//! V5 GPS

use core::ffi::c_double;

use crate::V5_DeviceT;

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsRaw {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C, packed)]
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

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsQuaternion {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

unsafe extern "system" {
    pub unsafe fn vexDeviceGpsReset(device: V5_DeviceT);
    pub unsafe fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceGpsQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceGpsQuaternion);
    pub unsafe fn vexDeviceGpsAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceGpsAttitude, bRaw: bool);
    pub unsafe fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw);
    pub unsafe fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw);
    pub unsafe fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32);
    pub unsafe fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32);
    pub unsafe fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double);
    pub unsafe fn vexDeviceGpsOriginGet(device: V5_DeviceT, ox: *mut c_double, oy: *mut c_double);
    pub unsafe fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double);
    pub unsafe fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceGpsInitialPositionSet(
        device: V5_DeviceT,
        initial_x: c_double,
        initial_y: c_double,
        initial_rotation: c_double,
    );
    pub unsafe fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double;
}
