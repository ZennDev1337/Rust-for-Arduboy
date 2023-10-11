//! This is the Module to interact in a save way with the Arduino C++ library.
use core::ffi::{c_long, c_ulong};

extern "C" {
    #[link_name = "arduino_random_between"]
    fn arduino_random_between_raw(min: c_long, max: c_long) -> c_long;

    #[link_name = "arduino_random_less_than"]
    fn arduino_random_less_than_raw(max: c_long) -> c_long;

    #[link_name = "arduino_delay"]
    fn arduino_delay(ms: c_ulong);
}
/// A Arduino function to get a random number between 2 numbers
/// seed based
pub fn random_between(min: i32, max: i32) -> i32 {
    unsafe { arduino_random_between_raw(min, max) }
}
/// A Arduino function to get a random number smaller than the number given
/// seed based
pub fn random_less_than(max: i32) -> i32 {
    unsafe { arduino_random_less_than_raw(max) }
}
/// A Arduino function to pause the cpu circles for a given amount of ms
pub fn delay(ms: u32) {
    unsafe { arduino_delay(ms) }
}
