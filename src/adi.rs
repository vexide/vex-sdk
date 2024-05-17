//! ADI Devices
use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_AdiPortConfiguration(pub core::ffi::c_uchar);

impl V5_AdiPortConfiguration {
    pub const kAdiPortTypeAnalogIn: Self = Self(0);
    pub const kAdiPortTypeAnalogOut: Self = Self(1);
    pub const kAdiPortTypeDigitalIn: Self = Self(2);
    pub const kAdiPortTypeDigitalOut: Self = Self(3);
    pub const kAdiPortTypeSmartButton: Self = Self(4);
    pub const kAdiPortTypeSmartPot: Self = Self(5);
    pub const kAdiPortTypeLegacyButton: Self = Self(6);
    pub const kAdiPortTypeLegacyPotentiometer: Self = Self(7);
    pub const kAdiPortTypeLegacyLineSensor: Self = Self(8);
    pub const kAdiPortTypeLegacyLightSensor: Self = Self(9);
    pub const kAdiPortTypeLegacyGyro: Self = Self(10);
    pub const kAdiPortTypeLegacyAccelerometer: Self = Self(11);
    pub const kAdiPortTypeLegacyServo: Self = Self(12);
    pub const kAdiPortTypeLegacyPwm: Self = Self(13);
    pub const kAdiPortTypeQuadEncoder: Self = Self(14);
    pub const kAdiPortTypeSonar: Self = Self(15);
    pub const kAdiPortTypeLegacyPwmSlew: Self = Self(16);
    pub const kAdiPortTypeUndefined: Self = Self(255);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceBumperState(pub core::ffi::c_uchar);

impl V5_DeviceBumperState {
    pub const kBumperReleased: Self = Self(0);
    pub const kBumperPressed: Self = Self(1);
}

map_jump_table! {
    0x208 => pub fn vexDeviceAdiPortConfigSet(device: V5_DeviceT, port: u32, config: V5_AdiPortConfiguration),
    0x20c => pub fn vexDeviceAdiPortConfigGet(device: V5_DeviceT, port: u32) -> V5_AdiPortConfiguration,
    0x210 => pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32),
    0x214 => pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32,
    0x21c =>
        /// <Derived from <https://github.com/purduesigbots/pros/blob/89a7417352fbbc86420325afe410861e2210743c/src/devices/vdml_ext_adi.c#L40>
        pub fn vexDeviceAdiAddrLedSet(device: V5_DeviceT, port: u32, pData: *mut u32, nOffset: u32, nLength: u32, options: u32),
    0x230 => pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState,
    0x258 => pub fn vexDeviceGyroReset(device: V5_DeviceT),
    0x25c => pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double,
    0x260 => pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double,
    0x280 => pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32,
}
