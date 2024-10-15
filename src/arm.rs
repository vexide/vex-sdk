//! CTE Workcell Arm

#[cfg(any(feature = "v5", feature = "exp"))]
use core::ffi::c_double;

#[cfg(any(feature = "v5", feature = "exp"))]
use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceArmTipPosition {
    pub tip_x: i32,
    pub tip_y: i32,
    pub tip_z: i32,
    pub tip_roll: i32,
    pub tip_pitch: i32,
    pub tip_yaw: i32,
    pub pose: i8,
    pub velocity: i16,
}

#[cfg(any(feature = "v5", feature = "exp"))]
map_jump_table! {
    0xb54 => pub fn vexDeviceArmMoveTipCommandLinearAdv(device: V5_DeviceT, position: *mut V5_DeviceArmTipPosition, j6_rotation: c_double, j6_velocity: u16, relative: bool),
    0xb58 => pub fn vexDeviceArmMoveTipCommandJointAdv(device: V5_DeviceT, position: *mut V5_DeviceArmTipPosition, j6_rotation: c_double, j6_velocity: u16, relative: bool),
    0xb5c => pub fn vexDeviceArmTipPositionGetAdv(device: V5_DeviceT, position: *mut V5_DeviceArmTipPosition),
    0xc30 => pub fn vexDeviceArmPoseSet(device: V5_DeviceT, pose: u8, velocity: u16),
    0xc34 => pub fn vexDeviceArmMoveTipCommandLinear(device: V5_DeviceT, x: i32, y: i32, z: i32, pose: u8, velocity: u16, rotation: c_double, rot_velocity: u16, relative: bool),
    0xc38 => pub fn vexDeviceArmMoveTipCommandJoint(device: V5_DeviceT, x: i32, y: i32, z: i32, pose: u8, velocity: u16, rotation: c_double, rot_velocity: u16, relative: bool),
    0xc3c => pub fn vexDeviceArmMoveJointsCommand(device: V5_DeviceT, positions: *mut c_double, velocities: *mut u16, j6_rotation: c_double, j6_velocity: u16, j7_volts: c_double, j7_timeout: u16, j7_i_limit: u16, relative: bool),
    0xc40 => pub fn vexDeviceArmSpinJoints(device: V5_DeviceT, velocities: *mut c_double),
    0xc44 => pub fn vexDeviceArmSetJointPositions(device: V5_DeviceT, new_positions: *mut c_double),
    0xc48 => pub fn vexDeviceArmPickUpCommand(device: V5_DeviceT),
    0xc4c => pub fn vexDeviceArmDropCommand(device: V5_DeviceT),
    0xc50 => pub fn vexDeviceArmMoveVoltsCommand(device: V5_DeviceT, voltages: *mut c_double),
    0xc54 => pub fn vexDeviceArmFullStop(device: V5_DeviceT, brakeMode: u8),
    0xc58 => pub fn vexDeviceArmEnableProfiler(device: V5_DeviceT, enable: u8),
    0xc5c => pub fn vexDeviceArmProfilerVelocitySet(device: V5_DeviceT, linear_velocity: u16, joint_velocity: u16),
    0xc60 => pub fn vexDeviceArmSaveZeroValues(device: V5_DeviceT),
    0xc64 => pub fn vexDeviceArmForceZeroCommand(device: V5_DeviceT),
    0xc68 => pub fn vexDeviceArmClearZeroValues(device: V5_DeviceT),
    0xc6c => pub fn vexDeviceArmBootload(device: V5_DeviceT),
    0xc70 => pub fn vexDeviceArmTipPositionGet(device: V5_DeviceT, x: *mut i32, y: *mut i32, z: *mut i32),
    0xc74 => pub fn vexDeviceArmJointInfoGet(device: V5_DeviceT, positions: *mut c_double, velocities: *mut c_double, currents: *mut i32),
    0xc78 => pub fn vexDeviceArmJ6PositionGet(device: V5_DeviceT) -> c_double,
    0xc7c => pub fn vexDeviceArmBatteryGet(device: V5_DeviceT) -> i32,
    0xc80 => pub fn vexDeviceArmServoFlagsGet(device: V5_DeviceT, servoID: u32) -> i32,
    0xc84 => pub fn vexDeviceArmStatusGet(device: V5_DeviceT) -> u32,
    0xc88 => pub fn vexDeviceArmDebugGet(device: V5_DeviceT, id: i32) -> u32,
    0xc8c => pub fn vexDeviceArmJointErrorsGet(device: V5_DeviceT, errors: *mut u8),
    0xc90 => pub fn vexDeviceArmJ6PositionSet(device: V5_DeviceT, position: i16),
    0xc94 => pub fn vexDeviceArmStopJointsCommand(device: V5_DeviceT, brakeModes: *mut i16),
    0xc98 => pub fn vexDeviceArmReboot(device: V5_DeviceT),
    0xc9c => pub fn vexDeviceArmTipOffsetSet(device: V5_DeviceT, x: i32, y: i32, z: i32),
}
