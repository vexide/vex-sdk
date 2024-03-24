#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod adi;
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
pub mod magnet;
pub mod motor;
pub mod optical;
pub mod range;
pub mod serial;
pub mod system;
pub mod touch;
pub mod vision;
pub mod user;

pub use adi::*;
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
pub use magnet::*;
pub use motor::*;
pub use optical::*;
pub use range::*;
pub use serial::*;
pub use system::*;
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
            #[no_mangle] $vis unsafe extern "C" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
                unsafe {
                    (*((crate::JUMP_TABLE_START + $offset) as *const extern "C" fn($($arg_ty,)*) $(-> $ret)?))($($arg,)*)
                }
            }
        )+
    };
}
