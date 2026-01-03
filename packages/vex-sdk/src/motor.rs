//! V5 Smart Motor

use core::ffi::c_double;

use crate::device::V5_DeviceT;

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

unsafe extern "system" {
    pub unsafe fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32);
    pub unsafe fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode);
    pub unsafe fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode;
    pub unsafe fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32);
    pub unsafe fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32);
    pub unsafe fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool);
    pub unsafe fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits);
    pub unsafe fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits;
    pub unsafe fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode);
    pub unsafe fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode;
    pub unsafe fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double);
    pub unsafe fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut u32) -> i32;
    pub unsafe fn vexDeviceMotorPositionReset(device: V5_DeviceT);
    pub unsafe fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double;
    pub unsafe fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double);
    pub unsafe fn vexDeviceMotorAbsoluteTargetSet(device: V5_DeviceT, position: c_double, veloctiy: i32);
    pub unsafe fn vexDeviceMotorRelativeTargetSet(device: V5_DeviceT, position: c_double, velocity: i32);
    pub unsafe fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32);
    pub unsafe fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset);
    pub unsafe fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset;
    pub unsafe fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32);
    pub unsafe fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32);
    pub unsafe fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub unsafe fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub unsafe fn vexDeviceMotorExternalProfileSet(device: V5_DeviceT, position: c_double, velocity: i32);
}
