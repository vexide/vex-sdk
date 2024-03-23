use crate::jump::map_jump_table;
use crate::devices::V5DeviceHandle;

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuRaw {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ImuOrientationMode {
    ZUp = 0x00,
    ZDown = 0x10,
    XUp = 0x20,
    XDown = 0x30,
    YUp = 0x40,
    YDown = 0x50,
    Auto = 0x80,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImuData {
    pub orientation: ImuOrientationMode,
    pub rotation: ImuRaw,
    pub acceleration: ImuRaw,
    pub reset_timestamp: u32,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuAttitude {
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
}

map_jump_table! {
	0x410 => pub fn imu_reset(device: V5DeviceHandle),
	0x414 => pub fn imu_heading(device: V5DeviceHandle) -> f64,
	0x418 => pub fn imu_degrees(device: V5DeviceHandle) -> f64,
	0x41c => pub fn imu_quaternion(device: V5DeviceHandle, data: *mut ImuQuaternion),
	0x420 => pub fn imu_attitude(device: V5DeviceHandle, data: *mut ImuAttitude),
	0x424 => pub fn imu_raw_gyro(device: V5DeviceHandle, data: *mut ImuRaw),
	0x428 => pub fn imu_raw_accel(device: V5DeviceHandle, data: *mut ImuRaw),
	0x42c => pub fn imu_status(device: V5DeviceHandle) -> u32,
	0x438 => pub fn set_imu_mode(device: V5DeviceHandle, mode: u32),
	0x43c => pub fn imu_mode(device: V5DeviceHandle) -> u32,
	0x444 => pub fn set_imu_data_rate(device: V5DeviceHandle, rate: u32),
}
