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
    /// Returns the hue of the detected object on the interval `[0.0, 360.0)`
    pub fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double;
    /// Returns the saturation of the detected object on the interval `[0.0, 1.0]`
    pub fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double;
    /// Returns the brightness of the detected object on the interval `[0.0, 1.0]`
    pub fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double;
    /// Returns the proximity of the detectd object on the interval [0, 255].
    /// A value of 255 indicates that the object is very close, a value of 0 indicates no object was detected.
    pub fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32;
    /// Returns the RGB value of the detected value through the `data` parameter.
    pub fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb);
    /// Sets the PWM, which ranges from `[0, 100]`, of the sensor's LED.
    /// A PWM of 100 is full brightness, a PWM of 0 turns the LED off.
    pub fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32);
    /// Gets the PWM of the sensor's LED from `[0, 100]`.
    pub fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32;
    /// Returns the internal status code of the sensor.
    pub fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32;
    /// Returns the raw, unprocessed RGBC data from the sesnor, via the `data` parameter.
    pub fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw);
    // TODO: document
    pub fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32);
    // TODO: document
    pub fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32;
    /// Returns the direction of the gesture detected by the sensor, as well as additional info via the `pData` parameter.
    ///
    /// # Return values
    /// - A value of 1 is up
    /// - A value of 2 is down
    /// - A value of 3 is left
    /// - A value of 4 is right
    /// - Any other value indicates no gesture
    pub fn vexDeviceOpticalGestureGet(
        device: V5_DeviceT,
        pData: *mut V5_DeviceOpticalGesture,
    ) -> u32;
    /// Enables gesture tracking for the given optical sensor.
    pub fn vexDeviceOpticalGestureEnable(device: V5_DeviceT);
    /// Disables gesture tracking for the given optical sensor.
    pub fn vexDeviceOpticalGestureDisable(device: V5_DeviceT);
    // TODO: document
    pub fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32);
    /// Sets the integration time of the sensor, from 3ms to 712ms. The default value is 103ms.
    pub fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double);
    /// Returns the integration time of the sensor in ms, from 3ms to 712ms.
    pub fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double;
}
