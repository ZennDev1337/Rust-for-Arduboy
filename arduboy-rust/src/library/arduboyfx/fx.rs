//! Functions given by the ArduboyFX library.
//!
//! You can use the 'FX::' module to access the functions after the import of the prelude
//! ```
//! use arduboy_rust::prelude::*;
//!
//! fn setup() {
//!     FX::begin()
//! }
//! ```
//! You will need to uncomment the ArduboyFX_Library in the import_config.h file.
#![allow(non_upper_case_globals)]
use super::drawable_number::DrawableNumber;
use super::drawable_string::DrawableString;
use core::ffi::{c_int, c_size_t, c_uchar, c_uint, c_ulong};
pub fn begin() {
    unsafe { arduboyfx_begin() }
}
pub fn begin_data(datapage: u16) {
    unsafe { arduboyfx_begin_data(datapage) }
}
pub fn begin_data_save(datapage: u16, savepage: u16) {
    unsafe { arduboyfx_begin_data_save(datapage, savepage) }
}
pub fn display() {
    unsafe { arduboyfx_display() }
}
pub fn display_clear() {
    unsafe { arduboyfx_display_clear() }
}
pub fn draw_number(n: impl DrawableNumber, digits: i8) {
    n.draw(digits)
}
pub fn draw_string(string: impl DrawableString) {
    string.draw()
}
pub fn draw_char(c: u8) {
    unsafe { arduboyfx_draw_char(c) }
}
pub fn draw_bitmap(x: i16, y: i16, address: u32, frame: u8, mode: u8) {
    unsafe { arduboyfx_draw_bitmap(x, y, address, frame, mode) }
}
pub fn draw_frame(address: u32) -> u32 {
    unsafe { arduboyfx_draw_frame(address) }
}
pub fn draw_loaded_frame() -> u8 {
    unsafe { arduboyfx_draw_loaded_frame() }
}
pub fn set_frame(frame: u32, repeat: u8) {
    unsafe { arduboyfx_set_frame(frame, repeat) }
}
pub fn read_data_array(
    address: u32,
    index: u8,
    offset: u8,
    element_size: u8,
    buffer: *const u8,
    length: usize,
) {
    unsafe { arduboyfx_read_data_array(address, index, offset, element_size, buffer, length) }
}
pub fn set_cursor(x: i16, y: i16) {
    unsafe { arduboyfx_set_cursor(x, y) }
}
pub fn set_cursor_x(x: i16) {
    unsafe { arduboyfx_set_cursor_x(x) }
}
pub fn set_cursor_y(y: i16) {
    unsafe { arduboyfx_set_cursor_y(y) }
}
pub fn set_cursor_range(left: i16, wrap: i16) {
    unsafe { arduboyfx_set_cursor_range(left, wrap) }
}
pub fn set_font(address: u32, mode: u8) {
    unsafe { arduboyfx_set_font(address, mode) }
}
pub fn set_font_mode(mode: u8) {
    unsafe { arduboyfx_set_font_mode(mode) };
}
pub fn load_game_state<T>(your_struct: &mut T) -> u8 {
    let pointer = your_struct as *mut T;
    let object_pointer = pointer as *mut u8;
    let object_size = core::mem::size_of::<T>();

    unsafe { arduboyfx_load_game_state(object_pointer, object_size) }
}
pub fn save_game_state<T>(your_struct: &T) {
    let pointer = your_struct as *const T;
    let object_pointer = pointer as *const u8;
    let object_size = core::mem::size_of::<T>();

    unsafe { arduboyfx_save_game_state(object_pointer, object_size) }
}
extern "C" {
    #[link_name = "arduboyfx_begin"]
    fn arduboyfx_begin();
    #[link_name = "arduboyfx_begin_data"]
    fn arduboyfx_begin_data(datapage: c_uint);
    #[link_name = "arduboyfx_begin_data_save"]
    fn arduboyfx_begin_data_save(datapage: c_uint, savepage: c_uint);
    #[link_name = "arduboyfx_display"]
    fn arduboyfx_display();
    #[link_name = "arduboyfx_display_clear"]
    fn arduboyfx_display_clear();
    #[link_name = "arduboyfx_read_data_array"]
    fn arduboyfx_read_data_array(
        address: c_ulong,
        index: c_uchar,
        offset: c_uchar,
        element_size: c_uchar,
        buffer: *const c_uchar,
        length: c_size_t,
    );
    #[link_name = "arduboyfx_draw_bitmap"]
    fn arduboyfx_draw_bitmap(x: c_int, y: c_int, address: c_ulong, frame: c_uchar, mode: c_uchar);
    #[link_name = "arduboyfx_set_frame"]
    fn arduboyfx_set_frame(frame: c_ulong, repeat: c_uchar);
    #[link_name = "arduboyfx_draw_frame"]
    fn arduboyfx_draw_frame(address: c_ulong) -> c_ulong;
    #[link_name = "arduboyfx_draw_loaded_frame"]
    fn arduboyfx_draw_loaded_frame() -> c_uchar;
    #[link_name = "arduboyfx_set_cursor"]
    fn arduboyfx_set_cursor(x: c_int, y: c_int);
    #[link_name = "arduboyfx_set_cursor_x"]
    fn arduboyfx_set_cursor_x(x: c_int);
    #[link_name = "arduboyfx_set_cursor_y"]
    fn arduboyfx_set_cursor_y(y: c_int);
    #[link_name = "arduboyfx_set_font"]
    fn arduboyfx_set_font(address: c_ulong, mode: c_uchar);
    #[link_name = "arduboyfx_set_font_mode"]
    fn arduboyfx_set_font_mode(mode: c_uchar);
    #[link_name = "arduboyfx_set_cursor_range"]
    fn arduboyfx_set_cursor_range(left: c_int, wrap: c_int);
    #[link_name = "arduboyfx_draw_char"]
    fn arduboyfx_draw_char(c: c_uchar);
    #[link_name = "arduboyfx_load_game_state"]
    fn arduboyfx_load_game_state(object: *mut u8, size: usize) -> u8;
    #[link_name = "arduboyfx_save_game_state"]
    fn arduboyfx_save_game_state(object: *const u8, size: usize);

}
