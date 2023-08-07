#![cfg(target_arch = "avr")]
#![no_std]
#![feature(c_size_t)]
extern crate panic_halt;
mod hardware;
mod library;
pub mod prelude;
mod print;
pub use crate::library::arduboy::{Arduboy, Color, Pstring, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tone::Sound;
pub use crate::library::eeprom::EEPROM;
pub use crate::library::{arduboy_tone_pitch, c, sprites};

pub use hardware::buttons;
