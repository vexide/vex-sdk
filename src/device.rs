#![allow(non_camel_case_types)]

use core::ffi::{c_double, c_int};

use pros::devices::smart::motor::Gearset;

use crate::{
    adi::V5_AdiPortConfiguration, map_jump_table, V5ImuOrientationMode, V5MotorBrakeMode,
    V5MotorControlMode, V5MotorEncoderUnits, V5_DeviceImuRaw, V5_DeviceMotorPid,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PositionData {
    pub position: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpsData {
    pub offset_x: c_double,
    pub offset_y: c_double,

    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdiExpanderData {
    pub adi_types: [V5_AdiPortConfiguration; 8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalData {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub brightness: c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImuData {
    pub orientation: V5ImuOrientationMode,
    pub rotation: V5_DeviceImuRaw,
    pub acceleration: V5_DeviceImuRaw,
    pub reset_timestamp: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MotorData {
    pub brake_mode: V5MotorBrakeMode,
    pub control_mode: V5MotorControlMode,
    pub encoder_units: V5MotorEncoderUnits,
    pub gearing: Gearset,
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
    pub motor: MotorData,
    pub imu: ImuData,
    pub rotation: PositionData,
    pub distance: PositionData,
    pub optical: OpticalData,
    pub vision: (),
    pub gps: GpsData,
    pub adi_expander: AdiExpanderData,
}

/// Handle to a [`V5_Device`]
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum V5_DeviceType {
    NoSensor = 0,
    MotorSensor = 2,
    LedSensor = 3,
    AbsEncSensor = 4,
    CrMotorSensor = 5,
    ImuSensor = 6,
    DistanceSensor = 7,
    RadioSensor = 8,
    TetherSensor = 9,
    BrainSensor = 10,
    VisionSensor = 11,
    AdiSensor = 12,
    Res1Sensor = 13,
    Res2Sensor = 14,
    Res3Sensor = 15,
    OpticalSensor = 16,
    MagnetSensor = 17,
    GpsSensor = 20,
    BumperSensor = 0x40,
    GyroSensor = 0x46,
    SonarSensor = 0x47,
    GenericSensor = 128,
    GenericSerial = 129,
    UndefinedSensor = 255,
}

map_jump_table! {
    0x190 =>
        /// Get the number of devices plugged into the brain.
        pub fn vexDevicesGetNumber() -> u32,
    0x194 =>
        /// Get the number of devices of a specific type plugged into the brain.
        pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32,
    0x198 => pub fn vexDevicesGet() -> V5_Device,
    0x18c =>
        /// Get a handle to a specific device plugged into a specific port index.
        pub fn vexDeviceGetByIndex(index: u32) -> V5_Device,
    0x1a0 =>
        /// Get the status flags of a specific device.
        pub fn vexDeviceGetStatus(devices: *const V5_DeviceType) -> i32,
    0x1b0 =>
        /// Get the timestamp recorded by a device's internal clock.
        pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32,
    0x2a8 =>
        pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double,
    0x1b4 => pub fn vexDeviceButtonStateGet() -> c_int,
}
