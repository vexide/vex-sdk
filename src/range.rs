//! Rangefinder/Lidar Sensor
//!
//! This sensor is not sold by VEX.

#[cfg(any(target_env = "v5", target_env = "exp"))]
use crate::{map_jump_table, V5_DeviceT};

#[cfg(any(target_env = "v5", target_env = "exp"))]
map_jump_table! {
    0x4d8 =>
        /// Rangefinder/Lidar - actual API to be determined
        pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32,
}
