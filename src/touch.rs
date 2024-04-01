//! Brain Screen Touchscreen

use crate::map_jump_table;

pub mod V5_TouchEvent {
    pub type Type = core::ffi::c_uint;

    pub const kTouchEventRelease: Type = 0;
    pub const kTouchEventPress: Type = 1;
    pub const kTouchEventPressAuto: Type = 2;
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_TouchStatus {
    pub lastEvent: V5_TouchEvent::Type,
    pub lastXpos: i16,
    pub lastYpos: i16,
    pub pressCount: i32,
    pub releaseCount: i32,
}

map_jump_table! {
    0x960 => pub fn vexTouchUserCallbackSet(callback: unsafe extern "C" fn(V5_TouchEvent::Type, i32, i32)),
    0x964 => pub fn vexTouchDataGet(status: *mut V5_TouchStatus),
}
