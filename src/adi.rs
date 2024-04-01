//! ADI Devices
use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_AdiPortConfiguration(core::ffi::c_uint);

impl V5_AdiPortConfiguration {    
    pub const kAdiPortTypeAnalogIn: core::ffi::c_uint = 0;
    pub const kAdiPortTypeAnalogOut: core::ffi::c_uint = 1;
    pub const kAdiPortTypeDigitalIn: core::ffi::c_uint = 2;
    pub const kAdiPortTypeDigitalOut: core::ffi::c_uint = 3;
    pub const kAdiPortTypeSmartButton: core::ffi::c_uint = 4;
    pub const kAdiPortTypeSmartPot: core::ffi::c_uint = 5;
    pub const kAdiPortTypeLegacyButton: core::ffi::c_uint = 6;
    pub const kAdiPortTypeLegacyPotentiometer: core::ffi::c_uint = 7;
    pub const kAdiPortTypeLegacyLineSensor: core::ffi::c_uint = 8;
    pub const kAdiPortTypeLegacyLightSensor: core::ffi::c_uint = 9;
    pub const kAdiPortTypeLegacyGyro: core::ffi::c_uint = 10;
    pub const kAdiPortTypeLegacyAccelerometer: core::ffi::c_uint = 11;
    pub const kAdiPortTypeLegacyServo: core::ffi::c_uint = 12;
    pub const kAdiPortTypeLegacyPwm: core::ffi::c_uint = 13;
    pub const kAdiPortTypeQuadEncoder: core::ffi::c_uint = 14;
    pub const kAdiPortTypeSonar: core::ffi::c_uint = 15;
    pub const kAdiPortTypeLegacyPwmSlew: core::ffi::c_uint = 16;
    pub const kAdiPortTypeUndefined: core::ffi::c_uint = 255;
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceBumperState(core::ffi::c_uint);

impl V5_DeviceBumperState {    
    pub const kBumperReleased: core::ffi::c_uint = 0;
    pub const kBumperPressed: core::ffi::c_uint = 1;
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
