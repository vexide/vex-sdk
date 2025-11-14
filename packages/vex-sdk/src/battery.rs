//! V5 Smart Battery

use core::ffi::c_double;

unsafe extern "system" {
    /// Returns the voltage of the battery in millivolts.
    pub fn vexBatteryVoltageGet() -> i32;
    
    /// Returns the current of the battery in milliamps.
    pub fn vexBatteryCurrentGet() -> i32;
    
    /// Returns the temperature of the battery in Celsius.
    pub fn vexBatteryTemperatureGet() -> c_double;
    
    /// Returns the capacity of the battery on a scale of [0.0, 100.0].
    pub fn vexBatteryCapacityGet() -> c_double;
}
