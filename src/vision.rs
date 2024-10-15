//! V5 Vision Sensor

#[cfg(any(target_env = "v5", target_env = "exp"))]
use crate::{map_jump_table, V5_DeviceT};

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5VisionMode(pub core::ffi::c_uchar);

impl V5VisionMode {
    pub const kVisionModeNormal: Self = Self(0);
    pub const kVisionModeMixed: Self = Self(1);
    pub const kVisionModeLineDetect: Self = Self(2);
    pub const kVisionTypeTest: Self = Self(3);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5VisionBlockType(pub core::ffi::c_uchar);

impl V5VisionBlockType {
    pub const kVisionTypeNormal: Self = Self(0);
    pub const kVisionTypeColorCode: Self = Self(1);
    pub const kVisionTypeLineDetect: Self = Self(2);
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceVisionObject {
    /// block signature
    pub signature: u16,
    /// block type
    pub r#type: V5VisionBlockType,
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

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceVisionSignature {
    pub id: u8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub range: f32,
    pub uMin: i32,
    pub uMax: i32,
    pub uMean: i32,
    pub vMin: i32,
    pub vMax: i32,
    pub vMean: i32,
    pub mRgb: u32,
    pub mType: u32,
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5VisionWBMode(pub core::ffi::c_uchar);

impl V5VisionWBMode {
    pub const kVisionWBNormal: Self = Self(0);
    pub const kVisionWBStart: Self = Self(1);
    pub const kVisionWBManual: Self = Self(2);
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceVisionRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5VisionLedMode(pub core::ffi::c_uchar);

impl V5VisionLedMode {
    pub const kVisionLedModeAuto: Self = Self(0);
    pub const kVisionLedModeManual: Self = Self(1);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5VisionWifiMode(pub core::ffi::c_uchar);

impl V5VisionWifiMode {
    pub const kVisionWifiModeOff: Self = Self(0);
    pub const kVisionWifiModeOn: Self = Self(1);
}

#[cfg(any(target_env = "v5", target_env = "exp"))]
map_jump_table! {
    0x398 => pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode),
    0x39c => pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode,
    0x3a0 => pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32,
    0x3a4 => pub fn vexDeviceVisionObjectGet(device: V5_DeviceT, index: u32, object: *mut V5_DeviceVisionObject) -> i32,
    0x3a8 => pub fn vexDeviceVisionSignatureSet(device: V5_DeviceT, signature: *mut V5_DeviceVisionSignature),
    0x3ac => pub fn vexDeviceVisionSignatureGet(device: V5_DeviceT, id: u32, signature: *mut V5_DeviceVisionSignature) -> bool,
    0x3c0 => pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8),
    0x3c4 => pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8,
    0x3c8 => pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode),
    0x3cc => pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode,
    0x3c0 => pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb),
    0x3c4 => pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb,
    0x3c8 => pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode),
    0x3cc => pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode,
    0x3d0 => pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8),
    0x3d4 => pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8,
    0x3d8 => pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb),
    0x3dc => pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb,
    0x3e0 => pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode),
    0x3e4 => pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode,
}
