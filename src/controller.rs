use crate::jump::map_jump_table;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerId {
    Primary = 0,
    Partner = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerChannel {
    JoystickLeftX = 0,
    JoystickLeftY,
    JoystickRightX,
    JoystickRightY,
    AnalogSpare1,
    AnalogSpare2,

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
		pub fn controller_channel(id: ControllerId, channel: ControllerChannel) -> i32,
	0x1a8 =>
		/// Returns `1` if the controller on the given ID is connected, or `0` if not.
		pub fn controller_connection_status(id: ControllerId) -> i32,
	0x1ac => pub fn set_controller_text(id: u32, line: u32, col: u32, buf: *const u8) -> u32,
}