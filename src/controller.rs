use crate::map_jump_table;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerId {
    kControllerMaster = 0,
    kControllerPartner,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerStatus {
    kV5ControllerOffline = 0,
    kV5ControllerTethered,
    kV5ControllerVexnet,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_ControllerIndex {
    Axis1 = 0,
    Axis2,
    Axis3,
    Axis4,

    ButtonL1,
    ButtonL2,
    ButtonR1,
    ButtonR2,

    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,

    ButtonX,
    ButtonB,
    ButtonY,
    ButtonA,

    ButtonSEL,

    BatteryLevel,

    ButtonAll,
    Flags,
    BatteryCapacity,
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
