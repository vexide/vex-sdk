//! Competition Control

#[cfg(any(feature = "v5", feature = "exp"))]
use crate::map_jump_table;

#[cfg(any(feature = "v5", feature = "exp"))]
map_jump_table! {
    0x9d8 => pub fn vexCompetitionStatus() -> u32,
    0x9dc => pub fn vexCompetitionControl(data: u32),
}
