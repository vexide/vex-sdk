//! V5 Smart Motor

use crate::{device::V5_DeviceT};

use core::ffi::c_double;

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5MotorBrakeMode(pub core::ffi::c_uchar);

impl V5MotorBrakeMode {
    pub const kV5MotorBrakeModeCoast: Self = Self(0);
    pub const kV5MotorBrakeModeBrake: Self = Self(1);
    pub const kV5MotorBrakeModeHold: Self = Self(2);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5MotorControlMode(pub core::ffi::c_uchar);

impl V5MotorControlMode {
    pub const kMotorControlModeOFF: Self = Self(0);
    pub const kMotorControlModeBRAKE: Self = Self(1);
    pub const kMotorControlModeHOLD: Self = Self(2);
    pub const kMotorControlModeSERVO: Self = Self(3);
    pub const kMotorControlModePROFILE: Self = Self(4);
    pub const kMotorControlModeVELOCITY: Self = Self(5);
    pub const kMotorControlModeUNDEFINED: Self = Self(6);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5MotorEncoderUnits(pub core::ffi::c_uchar);

impl V5MotorEncoderUnits {
    pub const kMotorEncoderDegrees: Self = Self(0);
    pub const kMotorEncoderRotations: Self = Self(1);
    pub const kMotorEncoderCounts: Self = Self(2);
}

#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5MotorGearset(pub core::ffi::c_uchar);

impl V5MotorGearset {
    pub const kMotorGearSet_36: Self = Self(0);
    pub const kMotorGearSet_18: Self = Self(1);
    pub const kMotorGearSet_06: Self = Self(2);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_DeviceMotorPid {
    pub kf: u8,
    pub kp: u8,
    pub ki: u8,
    pub kd: u8,
    pub filter: u8,
    pub pad1: u8,
    pub limit: u16,
    pub threshold: u8,
    pub loopspeed: u8,
    pub pad2: [u8; 2],
}
