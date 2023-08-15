#![cfg(target_arch = "avr")]
#![no_std]
#![feature(c_size_t)]

extern crate panic_halt;
pub mod hardware;
mod library;
pub mod prelude;
mod print;
pub use crate::library::arduboy2::{self, Arduboy2, Color, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tone::{self, ArduboyTones};
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
pub use crate::library::{c, sprites};
