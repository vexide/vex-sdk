use crate::{map_jump_table, V5_DeviceT};

map_jump_table! {
    0x4d8 =>
        /// Rangefinder/Lidar - actual API to be determined
        pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32,
}
