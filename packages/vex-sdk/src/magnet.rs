//! V5 Workcell Electromagnet

use core::ffi::c_double;

use crate::V5_DeviceT;

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceMagnetDuration(pub core::ffi::c_uchar);

impl V5_DeviceMagnetDuration {
    pub const kMagnetDurationShort: Self = Self(0);
    pub const kMagnetDurationMedium: Self = Self(1);
    pub const kMagnetDurationLong: Self = Self(2);
    pub const kMagnetDurationExtraLong: Self = Self(3);
}

unsafe extern "system" {
    pub unsafe fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32);
    pub unsafe fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    pub unsafe fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    pub unsafe fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32;
}
