use crate::jump::map_jump_table;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub hundredths: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: u16,
    pub day: u8,
    pub month: u8,
}

map_jump_table! {
	0x10 => pub fn stdlib_mismatch_error(param_1: u32, param_2: u32),
	0x20 => pub fn private_api_disable(),
	0x5c => 
		/// VEXos User Background Processing Procedure
		/// 
		/// This function handles transactions between the user processor and master
		/// processor when writing data to devices. This function should ideally be ran
		/// every 2mS for devices to be properly communicated with.
		pub fn background_processing(),
	// 0xf0 => pub fn debug(fmt: *const c_char, ...) -> i32,
	0x118 =>
		/// Gets the time since program start with millisecond precision.
		pub fn system_time() -> u32,
	0x11c => pub fn time() -> Time,
	0x120 => pub fn date() -> Date,
	0x124 => pub fn system_memory_dump(),
	0x130 =>
		/// Exits the current user program, returning to the main program screen.
		pub fn exit_request() -> !,
	0x134 =>
		/// Gets the time since program start with microsecond precision.
		pub fn high_res_time() -> u64,
	0x138 =>
		/// Gets the time since power on with microsecond precision.
		pub fn system_powerup_time() -> u64,
	0x13c =>
		/// Gets the address in memory of a linked file to the current user program, or `0`, if no file is linked.
		/// 
		/// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
		/// similar
		pub fn link_addr() -> u32,
	0x168 => pub fn system_timer(param_1: u32) -> u32,
	0x16c => pub fn system_timer_enable(param_1: u32) -> u32,
	0x170 => pub fn system_timer_disable(param_1: u32),
	0x174 => pub fn usb_status() -> u32,
	0x8c0 => pub fn system_timer_stop(),
	0x8c4 => pub fn system_timer_clear_interrupt(),
}
