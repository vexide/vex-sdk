//! V5 Rotation Sensor
//!
//! # Position vs Angle
//! The rotation sensor measures two angular values: position and angle. Position is unbounded and mesaured relative to the starting point of the sensor at program startup, but is reset across program runs. Angle is absolute and preserved across program runs.

use crate::V5_DeviceT;

unsafe extern "system" {
    /// Sets the position of the rotation sensor to the current angle.
    pub fn vexDeviceAbsEncReset(device: V5_DeviceT);
    /// Sets the position of the rotation sensor, in centidegrees.
    pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32);
    /// Returns the position of the rotation sensor, in centidegrees.
    pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32;
    /// Returns the angular velocity of the rotation sensor, in centidegrees / sec.
    pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32;
    /// Returns the angle of the rotation sensor, in centidegrees.
    pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32;
    /// Sets the reversed flag for the rotation sensor, where a value of `true` indicates the the sensor is reversed.
    pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool);
    /// Returns the reversed flag for the rotation sensor, where a value of `true` indicates the the sensor is reversed.
    pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool;
    /// Returns the internal status code of the rotation sensor.
    pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32;
    /// Sets the internal computation rate of the rotation sensor, in milliseconds. This does NOT affect the communication rate between the brain and sensor.
    ///
    /// The rate should be a multiple of 5ms and greater than or equal to 5 ms.
    pub fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32);
}
