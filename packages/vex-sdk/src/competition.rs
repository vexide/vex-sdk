//! Competition Control

unsafe extern "system" {
    /// Returns the competition state as a bitflag.
    ///
    /// The bitflag retuned by this function has the following layout:
    /// - Bit 0: Set if the robot is disabled via field control.
    /// - Bit 1: Set if the robot is in autonomous mode.
    /// - Bit 2: Set if the robot is connected to competition control (competition switch or field control).
    /// - Bit 3: set if the robot is connected to field control (NOT competition switch).
    pub fn vexCompetitionStatus() -> u32;
    
    pub fn vexCompetitionControl(data: u32);
}
