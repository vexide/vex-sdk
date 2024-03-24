use crate::map_jump_table;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum V5_TouchEvent {
    kTouchEventRelease,
    kTouchEventPress,
    kTouchEventPressAuto,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_TouchStatus {
    pub lastEvent: V5_TouchEvent,
    pub lastXpos: i16,
    pub lastYpos: i16,
    pub pressCount: i32,
    pub releaseCount: i32,
}

map_jump_table! {
    0x960 => pub fn vexTouchUserCallbackSet(callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32)),
    0x964 => pub fn vexTouchDataGet(status: *mut V5_TouchStatus),
}
