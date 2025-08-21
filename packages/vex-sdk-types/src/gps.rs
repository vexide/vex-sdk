//! V5 GPS

use core::ffi::c_double;

use crate::{V5_DeviceT};

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
