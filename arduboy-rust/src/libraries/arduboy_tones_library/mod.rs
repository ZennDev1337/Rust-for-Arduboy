//!This is the Module to interact in a save way with the ArduboyTones C++ library.
//!
//! You will need to uncomment the ArduboyTones_Library in the import_config.h file.
#[doc(hidden)]
mod arduboy_tones;
pub mod tones_pitch;

pub use arduboy_tones::ArduboyTones;
