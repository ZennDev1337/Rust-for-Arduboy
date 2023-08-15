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
//! Inside the Wrapper-Project folder is a folder named src.
//! You can disable C++ libraries in the main.h file.
//! Just comment the unused library definition out.
//!
//! To get an idea, the ArduboyTones Library needs 2-3% of the flash memory.
//!
//! [Here is the link to the GitHub Repo](https://github.com/zenndev1337/rust-for-arduboy)

extern crate panic_halt;
pub mod hardware;
mod library;
pub mod prelude;
mod print;
pub use crate::library::arduboy2::{self, Arduboy2, Color, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tone::{self, ArduboyTones};
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
pub use crate::library::{arduino, c, sprites};
