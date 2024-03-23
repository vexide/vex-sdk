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
	0x01c => pub fn scratch_memory_ptr(ptr: *mut *mut core::ffi::c_void) -> i32,
	0x20 => pub fn private_api_disable(),
	0x024 => pub fn private_api_enable(),
	0x028 =>
		/// Derived from https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17
		pub fn task_add(
			callback: unsafe extern "C" fn() -> core::ffi::c_int,
			interval: core::ffi::c_int,
			label: *const core::ffi::c_char
		),
	0x06c =>
		/// Derived from https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18
		pub fn task_sleep(time: u32),
	0xf74 => 
		/// VEXos User Background Processing Procedure
		/// 
		/// This function handles transactions between the user processor and master
		/// processor when writing data to devices. This function should ideally be ran
		/// every 2mS for devices to be properly communicated with.
		/// 
		/// TODO: Verify that this is at the right address
		pub fn background_processing(),
	0x118 =>
		/// Gets the time since program start with millisecond precision.
		pub fn system_time() -> u32,
	0x11c => pub fn time() -> Time,
	0x120 => pub fn date() -> Date,
	0x124 => pub fn system_memory_dump(), 
	0x128 => pub fn system_digital_io(pin: u32, value: u32),
	0x12c => pub fn system_startup_options() -> u32,
	0x130 =>
		/// Exits the current user program, returning to the main program screen.
		pub fn system_exit_request() -> !,
	0x134 =>
		/// Gets the time since program start with microsecond precision.
		pub fn system_high_res_time() -> u64,
	0x138 =>
		/// Gets the time since power on with microsecond precision.
		pub fn system_powerup_time() -> u64,
	0x13c =>
		/// Gets the address in memory of a linked file to the current user program, or `0`, if no file is linked.
		/// 
		/// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
		/// similar
		pub fn system_link_addr() -> u32,
	0x168 => pub fn system_timer(param_1: u32) -> u32,
	0x16c => pub fn system_timer_enable(param_1: u32) -> u32,
	0x170 => pub fn system_timer_disable(param_1: u32),
	0x174 => pub fn system_usb_status() -> u32,
	0x8c0 => pub fn system_timer_stop(),
	0x8c4 => pub fn system_timer_clear_interrupt(),
}