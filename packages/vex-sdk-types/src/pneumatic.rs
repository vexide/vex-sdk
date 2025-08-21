//! CTE Workcell Pneumatics Control

use crate::{V5_DeviceT};

#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_DevicePneumaticCtrl {
    pub flags: u16,
    pub m1_pwm: u8,
    pub m2_pwm: u8,
    pub m3_pwm: u8,
    pub m4_pwm: u8,
    pub m1_time: u8,
    pub m2_time: u8,
    pub m3_time: u8,
    pub m4_time: u8,
    pub comp_pwm: u8,
}
