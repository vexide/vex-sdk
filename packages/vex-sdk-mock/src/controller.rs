//! V5 Controller

pub use vex_sdk::{V5_ControllerId, V5_ControllerIndex, V5_ControllerStatus};

#[unsafe(no_mangle)]
pub extern "C" fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    Default::default()
}
