use crate::jump::map_jump_table;
use crate::types::{
    AdiExpanderData, GenericPositionData, GpsData, ImuData, MotorData, OpticalData,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceData {
    pub motor: MotorData,
    pub imu: ImuData,
    pub rotation: GenericPositionData,
    pub distance: GenericPositionData,
    pub optical: OpticalData,
    pub vision: (),
    pub gps: GpsData,
    pub adi_expander: AdiExpanderData,
}

pub type V5DeviceHandle = *mut V5Device;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct V5Device {
    pub port: u8,
    pub exists: bool,
    pub device_type: V5DeviceType,
    pub timestamp: u32,

    pub device_specific_data: DeviceData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum V5DeviceType {
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
        pub fn device_count() -> u32,
    0x194 =>
        /// Get the number of devices of a specific type plugged into the brain.
        pub fn device_type_count(device_type: V5DeviceType) -> u32,
    0x198 => pub fn devices() -> V5Device,
    0x18c =>
        /// Get a handle to a specific device plugged into a specific port index.
        pub fn device_by_index(index: u32) -> V5Device,
    0x1a0 =>
        /// Get the status flags of a specific device.
        pub fn device_status(devices: *const V5DeviceType) -> i32,
    0x1b0 =>
        /// Get the timestamp recorded by a device's internal clock.
        pub fn device_timestamp(device: V5DeviceHandle) -> u32,
}
