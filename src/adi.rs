//! ADI Devices
use core::ffi::c_double;

use crate::{map_jump_table, V5_DeviceT};

pub mod V5_AdiPortConfiguration {
    pub type Type = core::ffi::c_uint;
    
    pub const kAdiPortTypeAnalogIn: Type = 0;
    pub const kAdiPortTypeAnalogOut: Type = 1;
    pub const kAdiPortTypeDigitalIn: Type = 2;
    pub const kAdiPortTypeDigitalOut: Type = 3;
    pub const kAdiPortTypeSmartButton: Type = 4;
    pub const kAdiPortTypeSmartPot: Type = 5;
    pub const kAdiPortTypeLegacyButton: Type = 6;
    pub const kAdiPortTypeLegacyPotentiometer: Type = 7;
    pub const kAdiPortTypeLegacyLineSensor: Type = 8;
    pub const kAdiPortTypeLegacyLightSensor: Type = 9;
    pub const kAdiPortTypeLegacyGyro: Type = 10;
    pub const kAdiPortTypeLegacyAccelerometer: Type = 11;
    pub const kAdiPortTypeLegacyServo: Type = 12;
    pub const kAdiPortTypeLegacyPwm: Type = 13;
    pub const kAdiPortTypeQuadEncoder: Type = 14;
    pub const kAdiPortTypeSonar: Type = 15;
    pub const kAdiPortTypeLegacyPwmSlew: Type = 16;
    pub const kAdiPortTypeUndefined: Type = 255;
}

pub mod V5_DeviceBumperState {
    pub type Type = core::ffi::c_uint;
    
    pub const kBumperReleased: Type = 0;
    pub const kBumperPressed: Type = 1;
}

map_jump_table! {
    0x208 => pub fn vexDeviceAdiPortConfigSet(device: V5_DeviceT, port: u32, config: V5_AdiPortConfiguration::Type),
    0x20c => pub fn vexDeviceAdiPortConfigGet(device: V5_DeviceT, port: u32) -> V5_AdiPortConfiguration::Type,
    0x210 => pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32),
    0x214 => pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32,
    0x21c =>
        /// <Derived from <https://github.com/purduesigbots/pros/blob/89a7417352fbbc86420325afe410861e2210743c/src/devices/vdml_ext_adi.c#L40>
        pub fn vexDeviceAdiAddrLedSet(device: V5_DeviceT, port: u32, pData: *mut u32, nOffset: u32, nLength: u32, options: u32),
    0x230 => pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState::Type,
    0x258 => pub fn vexDeviceGyroReset(device: V5_DeviceT),
    0x25c => pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double,
    0x260 => pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double,
    0x280 => pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32,
}
