//! V5 Optical Sensor

use core::ffi::c_double;

use crate::{V5_DeviceT};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceOpticalRaw {
    pub clear: u16,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceOpticalRgb {
    pub red: c_double,
    pub green: c_double,
    pub blue: c_double,
    pub brightness: c_double,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceOpticalGesture {
    pub udata: u8,
    pub ddata: u8,
    pub ldata: u8,
    pub rdata: u8,
    pub gesture_type: u8,
    pub padding: u8,
    pub count: u16,
    pub time: u32,
}
