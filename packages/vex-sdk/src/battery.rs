//! V5 Smart Battery

use core::ffi::c_double;

unsafe extern "system" {
    pub safe fn vexBatteryVoltageGet() -> i32;
    pub safe fn vexBatteryCurrentGet() -> i32;
    pub safe fn vexBatteryTemperatureGet() -> c_double;
    pub safe fn vexBatteryCapacityGet() -> c_double;
}
