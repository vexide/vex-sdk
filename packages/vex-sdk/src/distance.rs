//! V5 Distance Sensor

use core::ffi::c_double;

use crate::V5_DeviceT;

unsafe extern "system" {
    /// Returns the distance of the detected object in mm in the range [0, 9999), or 9999 if no object is detected.
    pub fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32;
    /// Returns the confidence of the detected object, in the interval [0, 63].
    pub fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32;
    /// Returns the status code of the distance sensor.
    pub fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32;
    /// Returns the estimated size of the detected object in the interval [0, 400], or -1 if no object is detected.
    ///
    /// The return value of this function is unitless, and not fully understood. According to VEX, A 18" x 30" grey card will return a value of approximately 75 in typical room lighting.
    pub fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32;
    /// Returns the velocity of the object detected in m/s.
    pub fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double;
}
