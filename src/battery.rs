//! V5 Smart Battery

#[cfg(any(feature = "v5", feature = "exp"))]
use core::ffi::c_double;

#[cfg(any(feature = "v5", feature = "exp"))]
use crate::map_jump_table;

#[cfg(any(feature = "v5", feature = "exp"))]
map_jump_table! {
    0xa00 => pub fn vexBatteryVoltageGet() -> i32,
    0xa04 => pub fn vexBatteryCurrentGet() -> i32,
    0xa08 => pub fn vexBatteryTemperatureGet() -> c_double,
    0xa0c => pub fn vexBatteryCapacityGet() -> c_double,
}
