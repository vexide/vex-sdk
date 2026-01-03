//! V5 Optical Sensor

use core::ffi::c_double;

use crate::V5_DeviceT;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceOpticalRaw {
    pub clear: u16,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceOpticalRgb {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub brightness: c_double,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceOpticalGesture {
    pub udata: u8,
    pub ddata: u8,
    pub ldata: u8,
    pub rdata: u8,
    pub gesture_type: u8,
    pub padding: u8,
    pub count: u16,
    pub time: u32,
}

unsafe extern "system" {
    pub unsafe fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb);
    pub unsafe fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32);
    pub unsafe fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw);
    pub unsafe fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32);
    pub unsafe fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceOpticalGestureGet(
        device: V5_DeviceT,
        pData: *mut V5_DeviceOpticalGesture,
    ) -> u32;
    pub unsafe fn vexDeviceOpticalGestureEnable(device: V5_DeviceT);
    pub unsafe fn vexDeviceOpticalGestureDisable(device: V5_DeviceT);
    pub unsafe fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32);
    pub unsafe fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double);
    pub unsafe fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double;
}
