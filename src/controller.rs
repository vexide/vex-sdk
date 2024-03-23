use crate::jump::map_jump_table;
use crate::types::{ControllerId, ControllerChannel};

map_jump_table! {
	0x1a4 =>
		/// Get the value of a controller's data channel.
		pub fn controller_channel(id: ControllerId, channel: ControllerChannel) -> i32,
	0x1a8 =>
		/// Returns `1` if the controller on the given ID is connected, or `0` if not.
		pub fn controller_connection_status(id: ControllerId) -> i32,
	0x1ac => pub fn set_controller_text(id: u32, line: u32, col: u32, buf: *const u8) -> u32,
}