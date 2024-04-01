//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use crate::{
    adi::V5_AdiPortConfiguration, map_jump_table, V5ImuOrientationMode, V5MotorBrakeMode,
    V5MotorControlMode, V5MotorEncoderUnits, V5MotorGearset, V5_DeviceImuRaw, V5_DeviceMotorPid,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DevicePositionData {
    pub position: c_double,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceGpsData {
    pub offset_x: c_double,
    pub offset_y: c_double,

    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceAdiData {
    pub adi_types: [V5_AdiPortConfiguration; 8],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceOpticalData {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub brightness: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuData {
    pub orientation: V5ImuOrientationMode,
    pub rotation: V5_DeviceImuRaw,
    pub acceleration: V5_DeviceImuRaw,
    pub reset_timestamp: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceMotorData {
    pub brake_mode: V5MotorBrakeMode,
    pub control_mode: V5MotorControlMode,
    pub encoder_units: V5MotorEncoderUnits,
    pub gearing: V5MotorGearset,
    pub pos_pid: *mut V5_DeviceMotorPid,
    pub vel_pid: *mut V5_DeviceMotorPid,
    pub velocity_target: i32,
    pub velocity_max: i32,
    pub current: i32,
    pub current_max: i32,
    pub voltage: i32,
    pub voltage_max: i32,
    pub position: c_double,
    pub position_target: c_double,
    pub velocity: c_double,
    pub power: c_double,
    pub torque: c_double,
    pub efficiency: c_double,
    pub temperature: c_double,
    pub faults: u32,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union V5_DeviceData {
    pub motor: V5_DeviceMotorData,
    pub imu: V5_DeviceImuData,
    pub rotation: V5_DevicePositionData,
    pub distance: V5_DevicePositionData,
    pub optical: V5_DeviceOpticalData,
    pub vision: (),
    pub gps: V5_DeviceGpsData,
    pub adi: V5_DeviceAdiData,
}

/// Handle to a [`V5_Device`]
#[allow(non_camel_case_types)]
pub type V5_DeviceT = *mut V5_Device;

/// A device plugged into a smart port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct V5_Device {
    pub port: u8,
    pub exists: bool,
    pub device_type: V5_DeviceType,
    pub timestamp: u32,

    pub device_specific_data: V5_DeviceData,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_DeviceType(core::ffi::c_uint);

impl V5_DeviceType {
    pub const kDeviceTypeNoSensor: core::ffi::c_uint = 0;
    pub const kDeviceTypeMotorSensor: core::ffi::c_uint = 2;
    pub const kDeviceTypeLedSensor: core::ffi::c_uint = 3;
    pub const kDeviceTypeAbsEncSensor: core::ffi::c_uint = 4;
    pub const kDeviceTypeCrMotorSensor: core::ffi::c_uint = 5;
    pub const kDeviceTypeImuSensor: core::ffi::c_uint = 6;
    pub const kDeviceTypeDistanceSensor: core::ffi::c_uint = 7;
    pub const kDeviceTypeRadioSensor: core::ffi::c_uint = 8;
    pub const kDeviceTypeTetherSensor: core::ffi::c_uint = 9;
    pub const kDeviceTypeBrainSensor: core::ffi::c_uint = 10;
    pub const kDeviceTypeVisionSensor: core::ffi::c_uint = 11;
    pub const kDeviceTypeAdiSensor: core::ffi::c_uint = 12;
    pub const kDeviceTypeRes1Sensor: core::ffi::c_uint = 13;
    pub const kDeviceTypeRes2Sensor: core::ffi::c_uint = 14;
    pub const kDeviceTypeRes3Sensor: core::ffi::c_uint = 15;
    pub const kDeviceTypeOpticalSensor: core::ffi::c_uint = 16;
    pub const kDeviceTypeMagnetSensor: core::ffi::c_uint = 17;
    pub const kDeviceTypeGpsSensor: core::ffi::c_uint = 20;
    pub const kDeviceTypeAicameraSensor: core::ffi::c_uint = 26;
    pub const kDeviceTypeLightTowerSensor: core::ffi::c_uint = 27;
    pub const kDeviceTypeArmDevice: core::ffi::c_uint = 28;
    pub const kDeviceTypeAiVisionSensor: core::ffi::c_uint = 29;
    pub const kDeviceTypePneumaticSensor: core::ffi::c_uint = 30;
    pub const kDeviceTypeBumperSensor: core::ffi::c_uint = 0x40;
    pub const kDeviceTypeGyroSensor: core::ffi::c_uint = 0x46;
    pub const kDeviceTypeSonarSensor: core::ffi::c_uint = 0x47;
    pub const kDeviceTypeGenericSensor: core::ffi::c_uint = 128;
    pub const kDeviceTypeGenericSerial: core::ffi::c_uint = 129;
    pub const kDeviceTypeUndefinedSensor: core::ffi::c_uint = 255;
}

map_jump_table! {
    0x190 =>
        /// Get the number of devices plugged into the brain.
        pub fn vexDevicesGetNumber() -> u32,
    0x194 =>
        /// Get the number of devices of a specific type plugged into the brain.
        pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32,
    0x198 => pub fn vexDevicesGet() -> V5_DeviceT,
    0x19c =>
        /// Get a handle to a specific device plugged into a specific port index.
        pub fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT,
    0x1d8 =>
        /// Get a device's bitflags on a specific port index.
        /// 
        /// Function signature inferred based on return type of vex::device::flags.
        pub fn vexDeviceFlagsGetByIndex(index: u32) -> u32,
    0x1a0 =>
        /// Get a list of device types plugged into the brain.
        pub fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32,
    0x1b0 =>
        /// Get the timestamp recorded by a device's internal clock.
        pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32,
    0x2a8 =>
        pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double,
    0x1b4 => pub fn vexDeviceButtonStateGet() -> c_int,
}
