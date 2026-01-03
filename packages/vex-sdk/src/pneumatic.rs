//! CTE Workcell Pneumatics Control

use crate::V5_DeviceT;

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

unsafe extern "system" {
    pub unsafe fn vexDevicePneumaticActuationStatusGet(
        device: V5_DeviceT,
        ac1: *mut u16,
        ac2: *mut u16,
        ac3: *mut u16,
        ac4: *mut u16,
    ) -> u32;
    pub unsafe fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool);
    pub unsafe fn vexDevicePneumaticCtrlSet(device: V5_DeviceT, pCtrl: *mut V5_DevicePneumaticCtrl);
    pub unsafe fn vexDevicePneumaticCylinderPwmSet(device: V5_DeviceT, id: u32, bState: bool, pwm: u8);
    pub unsafe fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool);
    pub unsafe fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8);
    pub unsafe fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32;
}
