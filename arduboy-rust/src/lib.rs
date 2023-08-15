#![cfg(target_arch = "avr")]
#![no_std]
#![feature(c_size_t)]
//! This is the arduboy_rust crate
//! To get started import the [prelude] to your project.
//!
//! Import the module:
//! ```
//! use arduboy_rust::prelude::*;
//! ```

extern crate panic_halt;
pub mod hardware;
mod library;
pub mod prelude;
mod print;
pub use crate::library::arduboy2::{self, Arduboy2, Color, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tone::{self, ArduboyTones};
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
pub use crate::library::{arduino, c, sprites};
