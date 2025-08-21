//! ADI Devices

use core::ffi::c_double;

use crate::{V5_DeviceT};

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
