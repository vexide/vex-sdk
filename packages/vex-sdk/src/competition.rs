//! Competition Control

unsafe extern "system" {
    pub safe fn vexCompetitionStatus() -> u32;
    pub safe fn vexCompetitionControl(data: u32);
}
