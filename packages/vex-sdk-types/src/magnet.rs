//! V5 Workcell Electromagnet

use core::ffi::c_double;

use crate::{V5_DeviceT};

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceMagnetDuration(pub core::ffi::c_uchar);

impl V5_DeviceMagnetDuration {
    pub const kMagnetDurationShort: Self = Self(0);
    pub const kMagnetDurationMedium: Self = Self(1);
    pub const kMagnetDurationLong: Self = Self(2);
    pub const kMagnetDurationExtraLong: Self = Self(3);
}
