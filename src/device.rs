//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use crate::map_jump_table;

/// Handle to a [`V5_Device`]
#[allow(non_camel_case_types)]
pub type V5_DeviceT = *mut V5_Device;

/// A device plugged into a smart port
/// 
/// This private API type was derived from analysis of the bits returned
/// by [`vexDeviceGetByIndex`]. Not all fields are known.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_Device {
    pub zero_indexed_port: u8,
    _unknown0: u8,
    pub one_indexed_port: u8,
    pub device_type: V5_DeviceType, // this is 32 bit
    _unknown2: u8,
    pub installed: bool,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct V5_DeviceType(pub core::ffi::c_uint);

impl V5_DeviceType {
    pub const kDeviceTypeNoSensor: Self = Self(0);
    pub const kDeviceTypeMotorSensor: Self = Self(2);
    pub const kDeviceTypeLedSensor: Self = Self(3);
    pub const kDeviceTypeAbsEncSensor: Self = Self(4);
    pub const kDeviceTypeCrMotorSensor: Self = Self(5);
    pub const kDeviceTypeImuSensor: Self = Self(6);
    pub const kDeviceTypeDistanceSensor: Self = Self(7);
    pub const kDeviceTypeRadioSensor: Self = Self(8);
    pub const kDeviceTypeTetherSensor: Self = Self(9);
    pub const kDeviceTypeBrainSensor: Self = Self(10);
    pub const kDeviceTypeVisionSensor: Self = Self(11);
    pub const kDeviceTypeAdiSensor: Self = Self(12);
    pub const kDeviceTypeRes1Sensor: Self = Self(13);
    pub const kDeviceTypeRes2Sensor: Self = Self(14);
    pub const kDeviceTypeRes3Sensor: Self = Self(15);
    pub const kDeviceTypeOpticalSensor: Self = Self(16);
    pub const kDeviceTypeMagnetSensor: Self = Self(17);
    pub const kDeviceTypeGpsSensor: Self = Self(20);
    pub const kDeviceTypeAicameraSensor: Self = Self(26);
    pub const kDeviceTypeLightTowerSensor: Self = Self(27);
    pub const kDeviceTypeArmDevice: Self = Self(28);
    pub const kDeviceTypeAiVisionSensor: Self = Self(29);
    pub const kDeviceTypePneumaticSensor: Self = Self(30);
    pub const kDeviceTypeBumperSensor: Self = Self(0x40);
    pub const kDeviceTypeGyroSensor: Self = Self(0x46);
    pub const kDeviceTypeSonarSensor: Self = Self(0x47);
    pub const kDeviceTypeGenericSensor: Self = Self(128);
    pub const kDeviceTypeGenericSerial: Self = Self(129);
    pub const kDeviceTypeUndefinedSensor: Self = Self(255);
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
