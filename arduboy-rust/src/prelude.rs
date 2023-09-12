//! This is the important one to use this library effective in your project
//!
//! Import the module:
//! ```
//! use arduboy_rust::prelude::*;
//! ```
#[doc(inline)]
pub use crate::hardware::buttons::{self, *};
#[doc(inline)]
pub use crate::hardware::led::{self, *};
pub use crate::heapless::{LinearMap, String, Vec};
pub use crate::library::arduboy2::{self, *};
pub use crate::library::arduboy_tone::{self, ArduboyTones};
pub use crate::library::arduino::*;
pub use crate::library::ardvoice::*;
pub use crate::library::c::*;
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
#[doc(hidden)]
pub use crate::library::progmem::Pstring;
pub use crate::library::sprites;
pub use crate::print::*;
pub use crate::{f, get_sprite_addr, get_string_addr, get_tones_addr, progmem};
use core::cmp;
pub use core::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_size_t, c_uchar, c_uint, c_ulong,
    c_ulonglong,
};
#[doc(hidden)]
pub use core::ptr::addr_of;

pub fn constrain<T: Ord>(x: T, a: T, b: T) -> T {
    cmp::max(cmp::min(x, b), a)
}
