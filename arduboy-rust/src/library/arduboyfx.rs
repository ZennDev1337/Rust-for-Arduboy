//! This is the Module to interact in a save way with the ArduboyFX C++ library.
//!
//! You will need to uncomment the ArduboyFX_Library in the import_config.h file.
pub mod fx_consts;
mod drawable_number;
pub use drawable_number::DrawableNumber;
mod drawable_string;
pub use drawable_string::DrawableString;
pub mod fx;
