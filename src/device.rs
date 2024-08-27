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
    pub _unknown0: u8,
    pub one_indexed_port: u8,
    pub _unknown1_3: [u8; 3],
    pub device_type: V5_DeviceType, // this is 16 bit
    pub installed: bool,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct V5_DeviceType(pub core::ffi::c_ushort);

impl V5_DeviceType {
    /// No device connected
    pub const kDeviceTypeNoSensor: Self = Self(0);

    /// V5 Smart Motor
    pub const kDeviceTypeMotorSensor: Self = Self(2);

    /// Unknown use (possibly unreleased hardware)
    pub const kDeviceTypeLedSensor: Self = Self(3);

    /// Rotation Sensor
    pub const kDeviceTypeAbsEncSensor: Self = Self(4);

    /// V5 Motor CR (unknown use)
    pub const kDeviceTypeCrMotorSensor: Self = Self(5);

    /// Inertial Sensor
    pub const kDeviceTypeImuSensor: Self = Self(6);

    /// Distance Sensor
    pub const kDeviceTypeDistanceSensor: Self = Self(7);

    /// Radio
    pub const kDeviceTypeRadioSensor: Self = Self(8);

    /// Master Controller
    pub const kDeviceTypeTetherSensor: Self = Self(9);

    /// Brain
    pub const kDeviceTypeBrainSensor: Self = Self(10);

    /// Vision Sensor
    pub const kDeviceTypeVisionSensor: Self = Self(11);

    /// ADI
    pub const kDeviceTypeAdiSensor: Self = Self(12);

    /// Partner Controller
    pub const kDeviceTypeRes1Sensor: Self = Self(13);

    /// Battery
    pub const kDeviceTypeRes2Sensor: Self = Self(14);

    /// Solenoid (unknown use)
    pub const kDeviceTypeRes3Sensor: Self = Self(15);

    /// Optical Sensor
    pub const kDeviceTypeOpticalSensor: Self = Self(16);

    /// Electromagnet
    pub const kDeviceTypeMagnetSensor: Self = Self(17);

    /// GPS
    pub const kDeviceTypeGpsSensor: Self = Self(20);

    /// AI Stereo Camera
    pub const kDeviceTypeAicameraSensor: Self = Self(26);

    /// CTE Workcell Light Tower
    pub const kDeviceTypeLightTowerSensor: Self = Self(27);

    /// CTE Workcell Arm
    pub const kDeviceTypeArmDevice: Self = Self(28);

    /// AI Vision Sensor
    pub const kDeviceTypeAiVisionSensor: Self = Self(29);

    /// CTE Workcell Pneumatics
    pub const kDeviceTypePneumaticSensor: Self = Self(30);

    // All of these are probably just unreleased or beta hardware...
    pub const kDeviceTypeBumperSensor: Self = Self(0x40);
    pub const kDeviceTypeGyroSensor: Self = Self(0x46);
    pub const kDeviceTypeSonarSensor: Self = Self(0x47);
    pub const kDeviceTypeGenericSensor: Self = Self(128);

    /// Generic Serial
    pub const kDeviceTypeGenericSerial: Self = Self(129);

    /// Unknown use
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
