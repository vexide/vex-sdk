//! V5 Rotation Sensor

use crate::V5_DeviceT;

unsafe extern "system" {
    pub unsafe fn vexDeviceAbsEncReset(device: V5_DeviceT);
    pub unsafe fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32);
    pub unsafe fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32;
    pub unsafe fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool);
    pub unsafe fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool;
    pub unsafe fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32;
    pub unsafe fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32);
}
