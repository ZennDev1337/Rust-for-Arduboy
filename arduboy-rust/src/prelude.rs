//! This is the important one to use this library effective in your project
pub use crate::library::arduboy::Arduboy;
pub use core::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_size_t, c_uchar, c_uint, c_ulong,
    c_ulonglong,
};
#[doc(hidden)]
pub use core::ptr::addr_of;
///The main [Arduboy] struct ready to use in your project
#[allow(non_upper_case_globals)]
pub const arduboy: Arduboy = Arduboy {};
///The main [Sound] struct ready to use in your project
#[allow(non_upper_case_globals)]
pub const sound: Sound = Sound {};
pub use crate::hardware::buttons::*;
pub use crate::library::arduboy::{Color, Point, Rect, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tone::*;
pub use crate::library::arduino::*;
pub use crate::library::c::*;
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
pub use crate::library::progmem::Pstring;
pub use crate::library::sprites;
pub use crate::print::*;
pub use crate::{f, get_sprite_addr, get_string_addr, get_tones_addr, progmem};
use core::cmp;

pub fn constrain<T: Ord>(x: T, a: T, b: T) -> T {
    cmp::max(cmp::min(x, b), a)
}
