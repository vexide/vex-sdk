//! Brain Screen Touchscreen

pub use vex_sdk::{V5_TouchEvent, V5_TouchStatus};

#[unsafe(no_mangle)]
pub extern "C" fn vexTouchUserCallbackSet(callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32)) {
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn vexTouchDataGet(status: *mut V5_TouchStatus) {}
