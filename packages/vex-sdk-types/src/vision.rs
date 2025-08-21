//! V5 Vision Sensor

use crate::{V5_DeviceT};

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
