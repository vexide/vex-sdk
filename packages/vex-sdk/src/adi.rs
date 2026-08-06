//! ADI Devices

use core::ffi::c_double;

use crate::V5_DeviceT;

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_AdiPortConfiguration(pub core::ffi::c_uchar);

impl V5_AdiPortConfiguration {
    /// 12-bit generic analog input
    pub const kAdiPortTypeAnalogIn: Self = Self(0);
    /// 8-bit generic PWM output
    pub const kAdiPortTypeAnalogOut: Self = Self(1);
    /// Generic digital input
    pub const kAdiPortTypeDigitalIn: Self = Self(2);
    /// Generic digital output
    pub const kAdiPortTypeDigitalOut: Self = Self(3);
    /// V2 bumper switch
    pub const kAdiPortTypeSmartButton: Self = Self(4);
    /// V2 potentiometer
    pub const kAdiPortTypeSmartPot: Self = Self(5);
    /// Limit/bumper switch
    pub const kAdiPortTypeLegacyButton: Self = Self(6);
    /// Cortex-era potentiometer
    pub const kAdiPortTypeLegacyPotentiometer: Self = Self(7);
    /// Cortex-era line tracker
    pub const kAdiPortTypeLegacyLineSensor: Self = Self(8);
    /// Cortex-era light sensor
    pub const kAdiPortTypeLegacyLightSensor: Self = Self(9);
    /// Cortex-era gyroscope
    pub const kAdiPortTypeLegacyGyro: Self = Self(10);
    /// Cortex-era accelerometer
    pub const kAdiPortTypeLegacyAccelerometer: Self = Self(11);
    /// Cortex-era servo motor
    pub const kAdiPortTypeLegacyServo: Self = Self(12);
    /// MC29 controller output
    pub const kAdiPortTypeLegacyPwm: Self = Self(13);
    /// Quadrature encoder
    pub const kAdiPortTypeQuadEncoder: Self = Self(14);
    /// Ultrasonic sensor/sonar
    pub const kAdiPortTypeSonar: Self = Self(15);
    /// MC29 controller output with slew control
    pub const kAdiPortTypeLegacyPwmSlew: Self = Self(16);
    /// Undefined port (this type appears unused by VEXos)
    pub const kAdiPortTypeUndefined: Self = Self(255);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceBumperState(pub core::ffi::c_uchar);

impl V5_DeviceBumperState {
    pub const kBumperReleased: Self = Self(0);
    pub const kBumperPressed: Self = Self(1);
}

unsafe extern "system" {
    /// Sets the configuration for the given 0-indexed ADI port on the given ADI expander.
    pub fn vexDeviceAdiPortConfigSet(
        device: V5_DeviceT,
        port: u32,
        config: V5_AdiPortConfiguration,
    );
    /// Returns the set configuration for the given 0-indexed ADI port on the given ADI expander.
    pub fn vexDeviceAdiPortConfigGet(device: V5_DeviceT, port: u32) -> V5_AdiPortConfiguration;
    /// Sets the output value for the given 0-indexed ADI port on the given ADI expander.
    ///
    /// The range for this value depends on the configured device type.
    pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32);
    /// Returns the input value for the given 0-indexed ADI port on the given ADI expander.
    ///
    /// The range for this value depends on the configured device type.
    pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32;
    /// Sets the output value for the WS2812B LED strip on the given 0-indexed ADI port on the given ADI expander.
    ///
    /// # Parameters: 
    /// - `pData`: the buffer of `u32` color values in the format `0x00RRGGBB` for each pixel
    /// - `nOffset`: TODO
    /// - `nLength`: the length of the buffer
    /// - `options`: TODO
    pub fn vexDeviceAdiAddrLedSet(
        device: V5_DeviceT,
        port: u32,
        pData: *mut u32,
        nOffset: u32,
        nLength: u32,
        options: u32,
    );
    pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState;
    pub fn vexDeviceGyroReset(device: V5_DeviceT);
    pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32;
}
