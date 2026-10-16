//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use crate::V5_DeviceT;

unsafe extern "system" {
    /// Establishes a generic radio connection on the given port's device handle, with the given `link_id` and `type`.
    ///
    /// # Parameters
    /// - `link_id` is a unique identifier for the link that other radios can use to connect
    /// - `type` should be either 0 for a worker link or 1 for a manager link
    ///
    /// # Safety
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `link_id` must point to a valid null-terminated C-string.
    // TODO: what's ov?
    pub fn vexDeviceGenericRadioConnection(
        device: V5_DeviceT,
        link_id: *mut c_char,
        r#type: c_int,
        ov: bool,
    );
    /// Returns the number of bytes free in the radio's output buffer, or -1 on error.
    pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32;
    /// Writes the `u8` buffer `data` of length `size` to the given radio device, and returns the number of bytes written, or -1 on failure.
    ///
    /// # Safety
    ///
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-exhausive list of precondiditions that must be met for safety:
    /// - `data` must point to a readable buffer of `u8` at least `size` bytes long.
    pub fn vexDeviceGenericRadioTransmit(device: V5_DeviceT, data: *const u8, size: u16) -> i32;
    /// Returns the number of bytes available to read in the radio's input buffer, or -1 on error.
    pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> i32;
    /// Reads the available data from the given radio device into the provided buffer `data` of length `size`, and returns the number of bytes read, or -1 on failure.
    ///
    /// # Safety
    ///
    /// The safety of VEXos functions cannot be guaranteed. The following is a non-e
    /// - `data` must point to a writable buffer of `u8` at least `size` bytes long.
    pub fn vexDeviceGenericRadioReceive(device: V5_DeviceT, data: *mut u8, size: u16) -> i32;
    /// Returns `true` if the radio is connected to another radio, and false otherwise.
    pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool;
}
