//! V5 Smart Battery

use core::ffi::c_double;

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryVoltageGet() -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryCurrentGet() -> i32 {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryTemperatureGet() -> c_double {
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryCapacityGet() -> c_double {
    Default::default()
}
