use crate::{map_jump_table, V5DeviceHandle};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionMode {
    Normal = 0,
    Mixed = 1,
    LineDetect = 2,
    Test = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionBlockType {
    Normal = 0,
    ColorCode = 1,
    LineDetect = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionObject {
    /// block signature
    pub signature: u16,
    /// block type
    pub block_type: VisionBlockType,
    /// left side of block
    pub xoffset: u16,
    /// top of block
    pub yoffset: u16,
    /// width of block
    pub width: u16,
    /// height of block
    pub height: u16,
    /// angle of CC block in 0.1 deg units
    pub angle: u16,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionSignature {
    pub id: u8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub range: f32,
    pub u_min: i32,
    pub u_max: i32,
    pub u_mean: i32,
    pub v_min: i32,
    pub v_max: i32,
    pub v_mean: i32,
    pub m_rgb: u32,
    pub m_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWhiteBalanceMode {
    Normal = 0,
    Start = 1,
    Manual = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionLedMode {
    Auto = 0,
    Manual = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWifiMode {
    Off = 0,
    On = 1,
}

map_jump_table! {
	0x398 => pub fn set_vision_mode(device: V5DeviceHandle, mode: VisionMode),
	0x39c => pub fn vision_mode(device: V5DeviceHandle) -> VisionMode,
	0x3a0 => pub fn vision_object_count(device: V5DeviceHandle) -> i32,
	0x3a4 => pub fn vision_object(device: V5DeviceHandle, index: u32, object: *mut VisionObject) -> i32,
	0x3a8 => pub fn set_vision_signature(device: V5DeviceHandle, signature: *mut VisionSignature),
	0x3ac => pub fn vision_signature(device: V5DeviceHandle, id: u32, signature: *mut VisionSignature) -> bool,
	0x3c0 => pub fn set_vision_brightness(device: V5DeviceHandle, percent: u8),
	0x3c4 => pub fn vision_brightness(device: V5DeviceHandle) -> u8,
	0x3c8 => pub fn set_vision_white_balance_mode(device: V5DeviceHandle, mode: VisionWhiteBalanceMode),
	0x3cc => pub fn vision_white_balance_mode(device: V5DeviceHandle) -> VisionWhiteBalanceMode,
	0x3c0 => pub fn set_vision_white_balance(device: V5DeviceHandle, color: VisionRgb),
	0x3c4 => pub fn vision_white_balance(device: V5DeviceHandle) -> VisionRgb,
	0x3c8 => pub fn set_vision_led_mode(device: V5DeviceHandle, mode: VisionLedMode),
	0x3cc => pub fn vision_led_mode(device: V5DeviceHandle) -> VisionLedMode,
	0x3d0 => pub fn set_vision_led_brigntness(device: V5DeviceHandle, percent: u8),
	0x3d4 => pub fn vision_led_brigntness(device: V5DeviceHandle) -> u8,
	0x3d8 => pub fn set_vision_led_color(device: V5DeviceHandle, color: VisionRgb),
	0x3dc => pub fn vision_led_color(device: V5DeviceHandle) -> VisionRgb,
	0x3e0 => pub fn set_vision_wifi_mode(device: V5DeviceHandle, mode: VisionWifiMode),
	0x3e4 => pub fn vision_wifi_mode(device: V5DeviceHandle) -> VisionWifiMode,
}