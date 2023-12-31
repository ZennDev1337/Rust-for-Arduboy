//! This is the Module to interact in a save way with the ArduboyFX C++ library.
//!
//! You will need to uncomment the ArduboyFX in the config.toml file.
mod drawable_number;
pub mod fx_consts;
#[doc(hidden)]
pub use drawable_number::DrawableNumber;
mod drawable_string;
#[doc(hidden)]
pub use drawable_string::DrawableString;
pub mod fx;
