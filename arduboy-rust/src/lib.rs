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
//! ### Disable C++ libraries
//! Inside the root directory is a file named `import_config.h`
//!
//! You can disable C++ libraries in the `import_config.h` file.
//! Just comment the unused library definition out.
//!
//! Be careful with disabling libraries because:
//! - The only error you get is something like this if you try to use a function that relies on the library you disabled.
//! ```text
//! game.90c69b91e57f285-cgu.0:(.text.loop+0x290): undefined reference to `sound_tone'
//! ```
//! - the benefit of disabling will be important in the feature when I add support for the ArduboyG library etc.
//!
//! To get an idea, the ArduboyTones Library needs additional 2-3% of the flash memory.
//!
//! <a href="https://github.com/zenndev1337/rust-for-arduboy" target="_blank">Here is the link to the GitHub Repo</a>

extern crate panic_halt;
pub mod hardware;
mod library;
pub mod prelude;
mod print;
#[doc(inline)]
pub extern crate heapless;
pub use crate::library::arduboy2::{self, Arduboy2, Color, FONT_SIZE, HEIGHT, WIDTH};
pub use crate::library::arduboy_tones::{self, ArduboyTones};
pub use crate::library::ardvoice::{self, ArdVoice};
pub use crate::library::eeprom::{EEPROM, EEPROMBYTE};
pub use crate::library::{arduino, c, sprites};
pub use crate::library::arduboyfx::{self};
pub mod serial_print;
