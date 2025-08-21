//! V5 Smart Motor

use crate::map_jump_table;
use core::ffi::c_double;

use vex_sdk_types::V5_DeviceT;
pub use vex_sdk_types::{
    V5MotorBrakeMode, V5MotorControlMode, V5MotorEncoderUnits, V5MotorGearset, V5_DeviceMotorPid,
};

map_jump_table! {
    0x2d0 => pub fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32),
    0x2d4 => pub fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32,
    0x2d8 => pub fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double,
    0x2dc => pub fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32,
    0x2e0 => pub fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode),
    0x2e4 => pub fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode,
    0x2e8 => pub fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32),
    0x2ec => pub fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32,
    0x2f0 => pub fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32),
    0x2f4 => pub fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32,
    0x2f8 => pub fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32,
    0x2fc => pub fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double,
    0x300 => pub fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double,
    0x304 => pub fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double,
    0x308 => pub fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double,
    0x30c => pub fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool,
    0x310 => pub fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool,
    0x314 => pub fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool,
    0x318 => pub fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool,
    0x31c => pub fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool),
    0x320 => pub fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool,
    0x324 => pub fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits),
    0x328 => pub fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits,
    0x32c => pub fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode),
    0x330 => pub fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode,
    0x334 => pub fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double),
    0x338 => pub fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double,
    0x33c => pub fn vexDeviceMotorPositionRawGet(
        device: V5_DeviceT,
        timestamp: *mut u32,
    ) -> i32,
    0x340 => pub fn vexDeviceMotorPositionReset(device: V5_DeviceT),
    0x344 => pub fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double,
    0x348 => pub fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double),
    0x34c => pub fn vexDeviceMotorAbsoluteTargetSet(
        device: V5_DeviceT,
        position: c_double,
        veloctiy: i32,
    ),
    0x350 => pub fn vexDeviceMotorRelativeTargetSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    ),
    0x354 => pub fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32,
    0x358 => pub fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32,
    0x35c => pub fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32),
    0x360 => pub fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32,
    0x364 => pub fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset),
    0x368 => pub fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset,
    0x36c => pub fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32),
    0x370 => pub fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32,
    0x374 => pub fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32),
    0x378 => pub fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid),
    0x37c => pub fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid),
    0x380 => pub fn vexDeviceMotorExternalProfileSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    ),
}
