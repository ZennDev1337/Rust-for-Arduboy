//pub use crate::library::arduboy_tone_pitch::*;
use core::ffi::{c_uchar, c_uint, c_ulong};
///Create a `const` raw pointer to a sprite as u16, without creating an intermediate reference.
#[macro_export]
macro_rules! get_tones_addr {
    ( $s:expr ) => {
        addr_of!($s) as *const u16
    };
}
#[allow(unused_imports)]
pub(super) use get_tones_addr;

extern "C" {
    #[link_name = "sound_tone"]
    fn sound_tone(frequency: c_uint, duration: c_ulong);
    #[link_name = "sound_tone2"]
    fn sound_tone2(frequency1: c_uint, duration1: c_ulong, frequency2: c_uint, duration2: c_ulong);
    #[link_name = "sound_tone3"]
    fn sound_tone3(
        frequency1: c_uint,
        duration1: c_ulong,
        frequency2: c_uint,
        duration2: c_ulong,
        frequency3: c_uint,
        duration3: c_ulong,
    );

    #[link_name = "sound_tones"]
    fn sound_tones(tones: *const c_uint);

    #[link_name = "sound_no_tone"]
    fn sound_no_tone();

    #[link_name = "sound_playing"]
    fn sound_playing() -> bool;

    #[link_name = "sound_tones_in_ram"]
    fn sound_tones_in_ram(tones: *mut c_ulong);

    #[link_name = "sound_volume_mode"]
    fn sound_volume_mode(mode: c_uchar);

}
///This is the struct to interact in a save way with the Arduboy2Audio C++ library.
pub struct Sound {}
impl Sound {
    pub fn tone(&self, frequency: u16, duration: u32) {
        unsafe { sound_tone(frequency, duration) }
    }
    pub fn tone2(&self, frequency1: u16, duration1: u32, frequency2: u16, duration2: u32) {
        unsafe { sound_tone2(frequency1, duration1, frequency2, duration2) }
    }
    pub fn tone3(
        &self,
        frequency1: u16,
        duration1: u32,
        frequency2: u16,
        duration2: u32,
        frequency3: u16,
        duration3: u32,
    ) {
        unsafe { sound_tone3(frequency1, duration1, frequency2, duration2, frequency3, duration3) }
    }
    pub fn tones(&self, tones: *const u16) {
        unsafe { sound_tones(tones) }
    }
    pub fn no_tone(&self) {
        unsafe { sound_no_tone() }
    }
    pub fn playing(&self) -> bool {
        unsafe { sound_playing() }
    }
    pub fn tones_in_ram(&self, tones: *mut u32) {
        unsafe { sound_tones_in_ram(tones) }
    }
    pub fn volume_mode(&self, mode: u8) {
        unsafe { sound_volume_mode(mode) }
    }
}
