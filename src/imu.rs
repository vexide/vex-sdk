//! V5 Inertial Sensor

use core::ffi::c_double;

use crate::{device::V5_DeviceT, map_jump_table};

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

map_jump_table! {
    0x410 =>
        /// Calibrates the IMU. This function is non-blocking.
        pub fn vexDeviceImuReset(device: V5_DeviceT),
    0x414 =>
        /// Returns the yaw-axis rotation of the IMU as an unbounded angle in degrees.
        pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double,
    0x418 =>
        /// Returns the yaw-axis rotation of the IMU as an angle in degrees bounded 0-360.
        pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double,
    0x41c =>
        /// Returns quaternion defined by the IMU's rotation.
        pub fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion),
    0x420 =>
        /// Returns the 3-axis euler angles of the IMU bounded from -180 to 180 degrees.
        pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude),
    0x424 =>
        /// Returns the raw gyroscope readings of the IMU in degrees per second,
        pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw),
    0x428 =>
        /// Returns the raw accelerometer readings of the IMU in G.
        pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw),
    0x42c =>
        /// Returns the IMU's status bits.
        pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32,
    0x438 => pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32),
    0x43c => pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32,
    0x444 => pub fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32),
}
