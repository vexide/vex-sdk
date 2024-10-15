#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(c_variadic)]
#![cfg_attr(target_env = "rustc-dep-of-std", feature(link_cfg, no_core))]

#[cfg(all(target_env = "v5", target_env = "exp"))]
compile_error!("feature \"v5\" and feature \"exp\" cannot be enabled at the same time");

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

#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use abs_enc::*;
pub use adi::*;
pub use ai_vision::*;
pub use arm::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use battery::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use competition::*;
pub use controller::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use device::*;
pub use display::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use distance::*;
pub use file::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use generic_radio::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use generic_serial::*;
pub use gps::*;
pub use imu::*;
pub use led::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use light_tower::*;
pub use magnet::*;
pub use motor::*;
pub use optical::*;
pub use pneumatic::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use range::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use serial::*;
pub use system::*;
#[cfg(any(target_env = "v5", target_env = "exp"))]
pub use task::*;
pub use touch::*;
pub use vision::*;

#[cfg(target_env = "v5")]
pub const JUMP_TABLE_START: usize = 0x037fc000;

#[cfg(target_env = "exp")]
pub const JUMP_TABLE_START: usize = 0x301fc000;

#[macro_export]
#[cfg(any(target_env = "v5", target_env = "exp"))]
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
            $vis unsafe extern "aapcs" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
                unsafe {
                    (*(($crate::JUMP_TABLE_START + $offset) as *const extern "aapcs" fn($($arg_ty,)*) $(-> $ret)?))($($arg,)*)
                }
            }
        )+
    };
}
