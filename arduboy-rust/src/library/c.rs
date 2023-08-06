//! Clib functions you can use on the Arduboy
use core::ffi::{c_char, c_size_t};

extern "C" {
    #[link_name = "strlen"]
    fn c_strlen(cstr: *const c_char) -> c_size_t;
}
/// A C function to get the length of a string
pub fn strlen(cstr: *const i8) -> usize {
    unsafe { c_strlen(cstr) }
}
