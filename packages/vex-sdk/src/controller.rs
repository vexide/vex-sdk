//! V5 Controller

use crate::map_jump_table;

pub use vex_sdk_types::{V5_ControllerId, V5_ControllerIndex, V5_ControllerStatus};

map_jump_table! {
    0x1a4 =>
        /// Get the value of a controller's data channel.
        pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32,
    0x1a8 =>
        /// Returns `1` if the controller on the given ID is connected, or `0` if not.
        pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus,
    0x1ac => pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32,
}
