//! CTE Workcell Arm

use core::ffi::c_double;

use crate::V5_DeviceT;

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

unsafe extern "system" {
    pub unsafe fn vexDeviceArmMoveTipCommandLinearAdv(
        device: V5_DeviceT,
        position: *mut V5_DeviceArmTipPosition,
        j6_rotation: c_double,
        j6_velocity: u16,
        relative: bool,
    );
    pub unsafe fn vexDeviceArmMoveTipCommandJointAdv(
        device: V5_DeviceT,
        position: *mut V5_DeviceArmTipPosition,
        j6_rotation: c_double,
        j6_velocity: u16,
        relative: bool,
    );
    pub unsafe fn vexDeviceArmTipPositionGetAdv(
        device: V5_DeviceT,
        position: *mut V5_DeviceArmTipPosition,
    );
    pub unsafe fn vexDeviceArmPoseSet(device: V5_DeviceT, pose: u8, velocity: u16);
    pub unsafe fn vexDeviceArmMoveTipCommandLinear(
        device: V5_DeviceT,
        x: i32,
        y: i32,
        z: i32,
        pose: u8,
        velocity: u16,
        rotation: c_double,
        rot_velocity: u16,
        relative: bool,
    );
    pub unsafe fn vexDeviceArmMoveTipCommandJoint(
        device: V5_DeviceT,
        x: i32,
        y: i32,
        z: i32,
        pose: u8,
        velocity: u16,
        rotation: c_double,
        rot_velocity: u16,
        relative: bool,
    );
    pub unsafe fn vexDeviceArmMoveJointsCommand(
        device: V5_DeviceT,
        positions: *mut c_double,
        velocities: *mut u16,
        j6_rotation: c_double,
        j6_velocity: u16,
        j7_volts: c_double,
        j7_timeout: u16,
        j7_i_limit: u16,
        relative: bool,
    );
    pub unsafe fn vexDeviceArmSpinJoints(device: V5_DeviceT, velocities: *mut c_double);
    pub unsafe fn vexDeviceArmSetJointPositions(device: V5_DeviceT, new_positions: *mut c_double);
    pub unsafe fn vexDeviceArmPickUpCommand(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmDropCommand(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmMoveVoltsCommand(device: V5_DeviceT, voltages: *mut c_double);
    pub unsafe fn vexDeviceArmFullStop(device: V5_DeviceT, brakeMode: u8);
    pub unsafe fn vexDeviceArmEnableProfiler(device: V5_DeviceT, enable: u8);
    pub unsafe fn vexDeviceArmProfilerVelocitySet(
        device: V5_DeviceT,
        linear_velocity: u16,
        joint_velocity: u16,
    );
    pub unsafe fn vexDeviceArmSaveZeroValues(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmForceZeroCommand(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmClearZeroValues(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmBootload(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmTipPositionGet(device: V5_DeviceT, x: *mut i32, y: *mut i32, z: *mut i32);
    pub unsafe fn vexDeviceArmJointInfoGet(
        device: V5_DeviceT,
        positions: *mut c_double,
        velocities: *mut c_double,
        currents: *mut i32,
    );
    pub unsafe fn vexDeviceArmJ6PositionGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceArmBatteryGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceArmServoFlagsGet(device: V5_DeviceT, servoID: u32) -> i32;
    pub unsafe fn vexDeviceArmStatusGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceArmDebugGet(device: V5_DeviceT, id: i32) -> u32;
    pub unsafe fn vexDeviceArmJointErrorsGet(device: V5_DeviceT, errors: *mut u8);
    pub unsafe fn vexDeviceArmJ6PositionSet(device: V5_DeviceT, position: i16);
    pub unsafe fn vexDeviceArmStopJointsCommand(device: V5_DeviceT, brakeModes: *mut i16);
    pub unsafe fn vexDeviceArmReboot(device: V5_DeviceT);
    pub unsafe fn vexDeviceArmTipOffsetSet(device: V5_DeviceT, x: i32, y: i32, z: i32);
}
