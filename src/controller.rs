//! V5 Controller

use crate::map_jump_table;

pub mod V5_ControllerId {
    pub type Type = core::ffi::c_uint;
    
    pub const kControllerMaster: Type = 0;
    pub const kControllerPartner: Type = 1;
}

pub mod V5_ControllerStatus {
    pub type Type = core::ffi::c_uint;
    
    pub const kV5ControllerOffline: Type = 0;
    pub const kV5ControllerTethered: Type = 1;
    pub const kV5ControllerVexnet: Type = 2;
}

pub mod V5_ControllerIndex {
    pub type Type = core::ffi::c_uint;
    
    pub const Axis1: Type = 0;
    pub const Axis2: Type = 1;
    pub const Axis3: Type = 2;
    pub const Axis4: Type = 3;
    pub const ButtonL1: Type = 4;
    pub const ButtonL2: Type = 5;
    pub const ButtonR1: Type = 6;
    pub const ButtonR2: Type = 7;
    pub const ButtonUp: Type = 8;
    pub const ButtonDown: Type = 9;
    pub const ButtonLeft: Type = 10;
    pub const ButtonRight: Type = 11;
    pub const ButtonX: Type = 12;
    pub const ButtonB: Type = 13;
    pub const ButtonY: Type = 14;
    pub const ButtonA: Type = 15;
    pub const ButtonSEL: Type = 16;
    pub const BatteryLevel: Type = 17;
    pub const ButtonAll: Type = 18;
    pub const Flags: Type = 19;
    pub const BatteryCapacity: Type = 20;
}

map_jump_table! {
    0x1a4 =>
        /// Get the value of a controller's data channel.
        pub fn vexControllerGet(id: V5_ControllerId::Type, index: V5_ControllerIndex::Type) -> i32,
    0x1a8 =>
        /// Returns `1` if the controller on the given ID is connected, or `0` if not.
        pub fn vexControllerConnectionStatusGet(id: V5_ControllerId::Type) -> V5_ControllerStatus::Type,
    0x1ac => pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32,
}
