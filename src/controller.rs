//! V5 Controller

use crate::map_jump_table;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerId(pub core::ffi::c_uint);

impl V5_ControllerId {    
    pub const kControllerMaster: Self = Self(0);
    pub const kControllerPartner: Self = Self(1);
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerStatus(pub core::ffi::c_uint);

impl V5_ControllerStatus {    
    pub const kV5ControllerOffline: Self = Self(0);
    pub const kV5ControllerTethered: Self = Self(1);
    pub const kV5ControllerVexnet: Self = Self(2);
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct V5_ControllerIndex(pub core::ffi::c_uint);

impl V5_ControllerIndex {    
    pub const Axis1: Self = Self(0);
    pub const Axis2: Self = Self(1);
    pub const Axis3: Self = Self(2);
    pub const Axis4: Self = Self(3);
    pub const ButtonL1: Self = Self(4);
    pub const ButtonL2: Self = Self(5);
    pub const ButtonR1: Self = Self(6);
    pub const ButtonR2: Self = Self(7);
    pub const ButtonUp: Self = Self(8);
    pub const ButtonDown: Self = Self(9);
    pub const ButtonLeft: Self = Self(10);
    pub const ButtonRight: Self = Self(11);
    pub const ButtonX: Self = Self(12);
    pub const ButtonB: Self = Self(13);
    pub const ButtonY: Self = Self(14);
    pub const ButtonA: Self = Self(15);
    pub const ButtonSEL: Self = Self(16);
    pub const BatteryLevel: Self = Self(17);
    pub const ButtonAll: Self = Self(18);
    pub const Flags: Self = Self(19);
    pub const BatteryCapacity: Self = Self(20);
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
