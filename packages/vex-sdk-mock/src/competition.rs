//! Competition Control

#[unsafe(no_mangle)]
pub extern "C" fn vexCompetitionStatus() -> u32 {
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexCompetitionControl(data: u32) {}
