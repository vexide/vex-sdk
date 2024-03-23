pub const JUMP_TABLE_START: usize = 0x037FC000;

#[macro_export]
macro_rules! map_jump_table {
    (
        $(
            $offset:expr =>
            $(#[$meta:meta])* $vis:vis fn $name:ident($($arg:ident: $arg_ty:ty),* $(, ...)?) $(-> $ret:ty)?
        ),+ $(,)?
    ) => {
        $(
            $(#[$meta])*
            $vis unsafe extern "C" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
                unsafe {
                    (*((crate::jump::JUMP_TABLE_START + $offset) as *const extern "C" fn($($arg_ty,)*) $(-> $ret)?))($($arg,)*)
                }
            }
        )+
    };
}
pub use map_jump_table;