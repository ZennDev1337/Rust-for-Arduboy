//! This is the Module to interact in a save way with the ArdVoice C++ library.
//!
//! You will need to uncomment the ArdVoice_Library in the import_config.h file.

use core::ffi::{c_float, c_uchar, c_ulong};

extern "C" {
    #[link_name = "ardvoice_play_voice"]
    fn ardvoice_play_voice_raw(audio: *const c_uchar);
    #[link_name = "ardvoice_play_voice_complex"]
    fn ardvoice_play_voice_complex_raw(
        audio: *const c_uchar,
        startTime: c_ulong,
        endTime: c_ulong,
        speed: c_float,
    );
    #[link_name = "ardvoice_stop_voice"]
    fn ardvoice_stop_voice_raw();
    #[link_name = "ardvoice_is_voice_playing"]
    fn ardvoice_is_voice_playing_raw() -> bool;
}
///This is the struct to interact in a save way with the ArdVoice C++ library.
///
///You will need to uncomment the ArdVoice_Library in the import_config.h file.
pub struct ArdVoice {}
impl ArdVoice {
    pub const fn new() -> Self {
        ArdVoice {}
    }
    pub fn play_voice(&self, audio: *const u8) {
        unsafe { ardvoice_play_voice_raw(audio) }
    }
    pub fn play_voice_complex(&self, audio: *const u8, start_time: u32, end_time: u32, speed: f32) {
        unsafe { ardvoice_play_voice_complex_raw(audio, start_time, end_time, speed) }
    }
    pub fn stop_voice(&self) {
        unsafe { ardvoice_stop_voice_raw() }
    }
    pub fn is_voice_playing(&self) -> bool {
        unsafe { ardvoice_is_voice_playing_raw() }
    }
}
