use crate::jump::map_jump_table;
use crate::devices::V5DeviceHandle;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorBrakeMode {
    Coast = 0, // Motor will coast when stopped
    Brake = 1, // Motor will brake when stopped
    Hold = 2,  // Motor will hold position when stopped
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorControlMode {
    Off = 0,      // Motor is off and in coast mode
    Brake = 1,    // Motor is off and in brake mode
    Hold = 2,     // Motor is holding at current position
    Servo = 3,    // Motor is in "Servo" mode. Move to position and hold at that position
    Profile = 4,  // Motor moves to set position and stops.
    Velocity = 5, // Motor is unlimited movement at set 'velocity'
    Undefined = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorEncoderUnits {
    Degrees = 0,   // degrees Encoder Count Mode
    Rotations = 1, // rotations Encoder Count Mode
    Counts = 2,    // Raw Encoder Count Mode
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorGearset {
    Gearing36 = 0, // 36:1 gear set, torque
    Gearing18 = 1, // 18:1 gear set, speed
    Gearing06 = 2, // 6:1 gear set, high speed
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct MotorPid {
    pub kf: u8,
    pub kp: u8,
    pub ki: u8,
    pub kd: u8,
    pub filter: u8,
    pub d1: u8,
    pub limit: u16,
    pub threshold: u8,
    pub loopspeed: u8,
    pub pad2: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MotorData {
    pub brake_mode: MotorBrakeMode,
    pub control_mode: MotorControlMode,
    pub encoder_units: MotorEncoderUnits,
    pub gearing: MotorGearset,
    pub pos_pid: *mut MotorPid,
    pub vel_pid: *mut MotorPid,
    pub velocity_target: i32,
    pub velocity_max: i32,
    pub current: i32,
    pub current_max: i32,
    pub voltage: i32,
    pub voltage_max: i32,
    pub position: f64,
    pub position_target: f64,
    pub velocity: f64,
    pub power: f64,
    pub torque: f64,
    pub efficiency: f64,
    pub temperature: f64,
    pub faults: u32,
    pub flags: u8,
}

map_jump_table! {
	0x2d0 => pub fn set_motor_velocity(device: V5DeviceHandle, velocity: i32),
	0x2d4 => pub fn motor_velocity(device: V5DeviceHandle) -> i32,
	0x2d4 => pub fn motor_actual_velocity(device: V5DeviceHandle) -> f64,
	0x2dc => pub fn motor_direction(device: V5DeviceHandle) -> i32,
	0x2e0 => pub fn set_motor_mode(device: V5DeviceHandle, mode: MotorControlMode),
	0x2e4 => pub fn motor_mode(device: V5DeviceHandle) -> MotorControlMode,
	0x2e8 => pub fn set_motor_pwm(device: V5DeviceHandle, pwm: i32),
	0x2ec => pub fn motor_pwm(device: V5DeviceHandle) -> i32,
	0x2f0 => pub fn set_motor_current_limit(device: V5DeviceHandle, limit: i32),
	0x2f4 => pub fn motor_current_limit(device: V5DeviceHandle) -> i32,
	0x2f8 => pub fn motor_current(device: V5DeviceHandle) -> i32,
	0x2fc => pub fn motor_power(device: V5DeviceHandle) -> f64,
	0x300 => pub fn motor_torque(device: V5DeviceHandle) -> f64,
	0x304 => pub fn motor_efficiency(device: V5DeviceHandle) -> f64,
	0x308 => pub fn motor_temperature(device: V5DeviceHandle) -> f64,
	0x30c => pub fn motor_over_temp_flag(device: V5DeviceHandle) -> bool,
	0x310 => pub fn motor_current_limit_flag(device: V5DeviceHandle) -> bool,
	0x314 => pub fn motor_zero_velocity_flag(device: V5DeviceHandle) -> bool,
	0x318 => pub fn motor_zero_position_flag(device: V5DeviceHandle) -> bool,
	0x31c => pub fn set_motor_reverse_flag(device: V5DeviceHandle, reverse: bool),
	0x320 => pub fn motor_reverse_flag(device: V5DeviceHandle) -> bool,
	0x324 => pub fn set_motor_encoder_units(device: V5DeviceHandle, units: MotorEncoderUnits),
	0x328 => pub fn motor_encoder_units(device: V5DeviceHandle) -> MotorEncoderUnits,
	0x32c => pub fn set_motor_brake_mode(device: V5DeviceHandle, mode: MotorBrakeMode),
	0x330 => pub fn motor_brake_mode(device: V5DeviceHandle) -> MotorBrakeMode,
	0x334 => pub fn set_motor_position(device: V5DeviceHandle, position: f64),
	0x338 => pub fn motor_position(device: V5DeviceHandle) -> f64,
	0x33c => pub fn motor_position_raw(
		device: V5DeviceHandle,
		timestamp: *mut u32,
	) -> i32,
	0x340 => pub fn motor_position_reset(device: V5DeviceHandle),
	0x344 => pub fn motor_target(device: V5DeviceHandle) -> f64,
	0x348 => pub fn set_motor_servo_target(device: V5DeviceHandle, position: f64),
	0x34c => pub fn set_motor_absolute_target(
		device: V5DeviceHandle,
		position: f64,
		veloctiy: i32,
	),
	0x350 => pub fn set_motor_relative_target(
		device: V5DeviceHandle,
		position: f64,
		velocity: i32,
	),
	0x354 => pub fn motor_faults(device: V5DeviceHandle) -> u32,
	0x358 => pub fn motor_flags(device: V5DeviceHandle) -> u32,
	0x35c => pub fn set_motor_voltage(device: V5DeviceHandle, voltage: i32),
	0x360 => pub fn motor_voltage(device: V5DeviceHandle) -> i32,
	0x364 => pub fn set_motor_gearing(device: V5DeviceHandle, gearset: MotorGearset),
	0x368 => pub fn motor_gearing(device: V5DeviceHandle) -> MotorGearset,
	0x36c => pub fn set_motor_voltage_limit(device: V5DeviceHandle, limit: i32),
	0x370 => pub fn motor_voltage_limit(device: V5DeviceHandle) -> i32,
	0x374 => pub fn motor_velocity_update(device: V5DeviceHandle, velocity: i32),
	0x378 => pub fn set_motor_position_pid(device: V5DeviceHandle, pid: *mut MotorPid),
	0x37c => pub fn set_motor_velocity_pid(device: V5DeviceHandle, pid: *mut MotorPid),
	0x380 => pub fn set_motor_external_profile(
		device: V5DeviceHandle,
		position: f64,
		velocity: i32,
	),
}
