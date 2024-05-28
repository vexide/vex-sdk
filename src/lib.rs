#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(c_variadic)]
#![cfg_attr(feature = "rustc-dep-of-std", feature(link_cfg, no_core))]

#[cfg(feature = "rustc-dep-of-std")]
extern crate rustc_std_workspace_core as core;

pub mod abs_enc;
pub mod adi;
pub mod ai_vision;
pub mod arm;
pub mod battery;
pub mod competition;
pub mod controller;
pub mod device;
pub mod display;
pub mod distance;
pub mod file;
pub mod generic_radio;
pub mod generic_serial;
pub mod gps;
pub mod imu;
pub mod led;
pub mod light_tower;
pub mod magnet;
pub mod motor;
pub mod optical;
pub mod pneumatic;
pub mod range;
pub mod serial;
pub mod system;
pub mod task;
pub mod touch;
pub mod vision;

pub use abs_enc::*;
pub use adi::*;
pub use ai_vision::*;
pub use arm::*;
pub use battery::*;
pub use competition::*;
pub use controller::*;
pub use device::*;
pub use display::*;
pub use distance::*;
pub use file::*;
pub use generic_radio::*;
pub use generic_serial::*;
pub use gps::*;
pub use imu::*;
pub use led::*;
pub use light_tower::*;
pub use magnet::*;
pub use motor::*;
pub use optical::*;
pub use pneumatic::*;
pub use range::*;
pub use serial::*;
pub use system::*;
pub use task::*;
pub use touch::*;
pub use vision::*;

pub const JUMP_TABLE_START: usize = 0x037FC000;

#[macro_export]
macro_rules! map_jump_table {
    (
        $(
            $offset:expr =>
            $(#[$meta:meta])* $vis:vis fn $name:ident($($arg:ident: $arg_ty:ty $(,)?),*) $(-> $ret:ty)?
        ),+ $(,)?
    ) => {
        $(
            $(#[$meta])*
            #[doc = "# Safety\nCalls to jumptable functions are unsafe because jumptable functions are owned by VEXos and we cannot guarantee their safety."]
            #[no_mangle] $vis unsafe extern "C" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
                unsafe {
                    (*(($crate::JUMP_TABLE_START + $offset) as *const extern "C" fn($($arg_ty,)*) $(-> $ret)?))($($arg,)*)
                }
            }
        )+
    };
}
