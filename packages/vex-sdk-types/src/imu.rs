//! V5 Inertial Sensor

use core::ffi::c_double;

use crate::device::V5_DeviceT;

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuRaw {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuQuaternion {
    pub a: c_double,
    pub b: c_double,
    pub c: c_double,
    pub d: c_double,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct V5ImuOrientationMode(pub core::ffi::c_uchar);

impl V5ImuOrientationMode {
    pub const kImuOrientationZUp: Self = Self(0x00);
    pub const kImuOrientationZDown: Self = Self(0x10);
    pub const kImuOrientationXUp: Self = Self(0x20);
    pub const kImuOrientationXDown: Self = Self(0x30);
    pub const kImuOrientationYUp: Self = Self(0x40);
    pub const kImuOrientationYDown: Self = Self(0x50);
    pub const kImuOrientationAuto: Self = Self(0x80);
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuAttitude {
    pub pitch: c_double,
    pub roll: c_double,
    pub yaw: c_double,
}
