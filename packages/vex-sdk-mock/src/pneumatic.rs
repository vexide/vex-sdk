//! CTE Workcell Pneumatics Control

pub use vex_sdk::V5_DevicePneumaticCtrl;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticActuationStatusGet(
    device: V5_DeviceT,
    ac1: *mut u16,
    ac2: *mut u16,
    ac3: *mut u16,
    ac4: *mut u16,
) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticCtrlSet(
    device: V5_DeviceT,
    pCtrl: *mut V5_DevicePneumaticCtrl,
) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticCylinderPwmSet(
    device: V5_DeviceT,
    id: u32,
    bState: bool,
    pwm: u8,
) {
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8) {}
#[unsafe(no_mangle)]
pub extern "C" fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32 {
    Default::default()
}
