//! Competition Control

#[cfg(any(target_env = "v5", target_env = "exp"))]
use crate::map_jump_table;

#[cfg(any(target_env = "v5", target_env = "exp"))]
map_jump_table! {
    0x9d8 => pub fn vexCompetitionStatus() -> u32,
    0x9dc => pub fn vexCompetitionControl(data: u32),
}
