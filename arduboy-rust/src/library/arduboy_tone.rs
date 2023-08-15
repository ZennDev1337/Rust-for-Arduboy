//!This is the Module to interact in a save way with the ArduboyTones C++ library.
pub use crate::library::arduboy_tone_pitch;
use core::ffi::{c_uchar, c_uint, c_ulong};
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
pub struct ArduboyTones {}
impl ArduboyTones {
    ///Get a new instance of [ArduboyTones]
    /// ## Example
    /// ```
    /// const sound: ArduboyTones = ArduboyTones::new();
    /// ```
    pub const fn new() -> ArduboyTones {
        ArduboyTones {}
    }
    ///Play a single tone.
    ///
    ///- freq The frequency of the tone, in hertz.
    ///- dur The duration to play the tone for, in 1024ths of a
    ///second (very close to milliseconds). A duration of 0, or if not provided,
    ///means play forever, or until `noTone()` is called or a new tone or
    ///sequence is started.
    pub fn tone(&self, frequency: u16, duration: u32) {
        unsafe { sound_tone(frequency, duration) }
    }
    /// Play two tones in sequence.
    ///
    /// - freq1,freq2 The frequency of the tone in hertz.
    /// - dur1,dur2 The duration to play the tone for, in 1024ths of a
    /// second (very close to milliseconds).
    pub fn tone2(&self, frequency1: u16, duration1: u32, frequency2: u16, duration2: u32) {
        unsafe { sound_tone2(frequency1, duration1, frequency2, duration2) }
    }
    /// Play three tones in sequence.
    ///
    /// - freq1,freq2,freq3 The frequency of the tone, in hertz.
    /// - dur1,dur2,dur3 The duration to play the tone for, in 1024ths of a
    /// second (very close to milliseconds).
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
    /// Play a tone sequence from frequency/duration pairs in a PROGMEM array.
    ///
    /// - tones A pointer to an array of frequency/duration pairs.
    ///
    /// The array must be placed in code space using `PROGMEM`.
    ///
    /// See the `tone()` function for details on the frequency and duration values.
    /// A frequency of 0 for any tone means silence (a musical rest).
    ///
    /// The last element of the array must be `TONES_END` or `TONES_REPEAT`.
    ///
    /// Example:
    /// ```
    /// progmem!(
    ///     static sound1:[u8;_]=[220,1000, 0,250, 440,500, 880,2000,TONES_END]
    /// );
    ///
    /// tones(get_tones_addr!(sound1));
    /// ```
    pub fn tones(&self, tones: *const u16) {
        unsafe { sound_tones(tones) }
    }
    /// Stop playing the tone or sequence.
    ///
    /// If a tone or sequence is playing, it will stop. If nothing
    /// is playing, this function will do nothing.
    pub fn no_tone(&self) {
        unsafe { sound_no_tone() }
    }
    /// Check if a tone or tone sequence is playing.
    ///
    /// - return boolean `true` if playing (even if sound is muted).
    pub fn playing(&self) -> bool {
        unsafe { sound_playing() }
    }
    /// Play a tone sequence from frequency/duration pairs in an array in RAM.
    ///
    /// - tones A pointer to an array of frequency/duration pairs.
    ///
    /// The array must be located in RAM.
    ///
    /// See the `tone()` function for details on the frequency and duration values.
    /// A frequency of 0 for any tone means silence (a musical rest).
    ///
    /// The last element of the array must be `TONES_END` or `TONES_REPEAT`.
    ///
    /// Example:
    ///
    /// ```
    /// let sound2: [u16; 9] = [220, 1000, 0, 250, 440, 500, 880, 2000, TONES_END];
    /// ```
    /// Using `tones()`, with the data in PROGMEM, is normally a better
    /// choice. The only reason to use tonesInRAM() would be if dynamically
    /// altering the contents of the array is required.
    pub fn tones_in_ram(&self, tones: *mut u32) {
        unsafe { sound_tones_in_ram(tones) }
    }
    /// Set the volume to always normal, always high, or tone controlled.
    ///
    /// One of the following values should be used:
    ///
    /// - `VOLUME_IN_TONE` The volume of each tone will be specified in the tone
    ///    itself.
    /// - `VOLUME_ALWAYS_NORMAL` All tones will play at the normal volume level.
    /// - `VOLUME_ALWAYS_HIGH` All tones will play at the high volume level.
    pub fn volume_mode(&self, mode: u8) {
        unsafe { sound_volume_mode(mode) }
    }
}
