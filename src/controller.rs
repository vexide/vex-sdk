//! V5 Controller

use crate::map_jump_table;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerId(core::ffi::c_uint);

impl V5_ControllerId {    
    pub const kControllerMaster: core::ffi::c_uint = 0;
    pub const kControllerPartner: core::ffi::c_uint = 1;
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerStatus(core::ffi::c_uint);

impl V5_ControllerStatus {    
    pub const kV5ControllerOffline: core::ffi::c_uint = 0;
    pub const kV5ControllerTethered: core::ffi::c_uint = 1;
    pub const kV5ControllerVexnet: core::ffi::c_uint = 2;
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerIndex(core::ffi::c_uint);

impl V5_ControllerIndex {    
    pub const Axis1: core::ffi::c_uint = 0;
    pub const Axis2: core::ffi::c_uint = 1;
    pub const Axis3: core::ffi::c_uint = 2;
    pub const Axis4: core::ffi::c_uint = 3;
    pub const ButtonL1: core::ffi::c_uint = 4;
    pub const ButtonL2: core::ffi::c_uint = 5;
    pub const ButtonR1: core::ffi::c_uint = 6;
    pub const ButtonR2: core::ffi::c_uint = 7;
    pub const ButtonUp: core::ffi::c_uint = 8;
    pub const ButtonDown: core::ffi::c_uint = 9;
    pub const ButtonLeft: core::ffi::c_uint = 10;
    pub const ButtonRight: core::ffi::c_uint = 11;
    pub const ButtonX: core::ffi::c_uint = 12;
    pub const ButtonB: core::ffi::c_uint = 13;
    pub const ButtonY: core::ffi::c_uint = 14;
    pub const ButtonA: core::ffi::c_uint = 15;
    pub const ButtonSEL: core::ffi::c_uint = 16;
    pub const BatteryLevel: core::ffi::c_uint = 17;
    pub const ButtonAll: core::ffi::c_uint = 18;
    pub const Flags: core::ffi::c_uint = 19;
    pub const BatteryCapacity: core::ffi::c_uint = 20;
}

map_jump_table! {
    0x1a4 =>
        /// Get the value of a controller's data channel.
        pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32,
    0x1a8 =>
        /// Returns `1` if the controller on the given ID is connected, or `0` if not.
        pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus,
    0x1ac => pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32,
}
