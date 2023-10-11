//! This is the important one to use this library effective in your project
//!
//! Import the module:
//! ```
//! use arduboy_rust::prelude::*;
//! ```
#[doc(inline)]
pub use crate::hardware::buttons::*;
#[doc(inline)]
pub use crate::hardware::led::*;
pub use crate::heapless::{LinearMap, String, Vec};
pub use crate::libraries::arduboy2_library::*;
pub use crate::libraries::arduboy_tones_library::*;
pub use crate::libraries::arduboyfx_library::*;
pub use crate::libraries::arduino_system::arduino::*;
pub use crate::libraries::arduino_system::c::*;
pub use crate::libraries::arduino_system::eeprom::*;
pub use crate::libraries::arduino_system::progmem::*;
pub use crate::libraries::arduino_system::serial_print as serial;
pub use crate::libraries::ardvoice_library::ArdVoice;
pub use crate::{
    f, get_ardvoice_tone_addr, get_sprite_addr, get_string_addr, get_tones_addr, progmem,
};
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
