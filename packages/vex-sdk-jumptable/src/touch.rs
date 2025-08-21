//! Brain Screen Touchscreen

pub use vex_sdk::{V5_TouchEvent, V5_TouchStatus};

use crate::map_jump_table;

map_jump_table! {
    0x960 => pub fn vexTouchUserCallbackSet(callback: unsafe extern "system" fn(V5_TouchEvent, i32, i32)),
    0x964 => pub fn vexTouchDataGet(status: *mut V5_TouchStatus),
}
