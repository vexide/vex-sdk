//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use crate::map_jump_table;

pub use vex_sdk_types::{V5_DeviceType, V5_DeviceT};

map_jump_table! {
    0x190 =>
        /// Get the number of device ports currently present on this system.
        ///
        /// As of VEXos 1.1.4, this number is the constant `23`.
        pub fn vexDevicesGetNumber() -> u32,
    0x194 =>
        /// Get the number of devices of a specific type plugged into the brain.
        pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32,
    0x198 =>
        /// Get a buffer of all devices on the brain.
        pub fn vexDevicesGet() -> V5_DeviceT,
    0x19c =>
        /// Get a handle to a device plugged into the specified port index.
        pub fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT,
    0x1a0 =>
        /// Get a list of device types plugged into the brain.
        ///
        /// Returns -1 if a null pointer is passed, otherwise the number of devices
        /// that were written to the buffer.
        ///
        /// The length of the buffer should be at most [`V5_MAX_DEVICE_PORTS`].
        pub fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32,
    0x1b0 =>
        /// Get the timestamp recorded by a device's internal clock.
        pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32,
    0x2a8 =>
        pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> c_double,
    0x1b4 => pub fn vexDeviceButtonStateGet() -> c_int,
}
