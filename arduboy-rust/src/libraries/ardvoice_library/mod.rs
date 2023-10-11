//! This is the Module to interact in a save way with the ArdVoice C++ library.
//!
//! You will need to uncomment the ArdVoice in the config.toml file.
mod ardvoice;
pub use ardvoice::ArdVoice;
