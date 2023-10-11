//! This is the Module to interact in a save way with the ArdVoice C++ library.
//!
//! You will need to uncomment the ArdVoice_Library in the import_config.h file.
mod ardvoice;
pub use ardvoice::ArdVoice;
