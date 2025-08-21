//! CTE Workcell Arm

use core::ffi::c_double;

use crate::{V5_DeviceT};

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
