//!This is the module to interact in a save way with the Sprites C++ library.
use core::ffi::{c_int, c_uchar};

extern "C" {
    #[link_name = "arduino_draw_override"]
    fn arduino_draw_override_raw(x: c_int, y: c_int, bitmap: *const c_uchar, frame: c_uchar);
    #[link_name = "arduino_draw_external_mask"]
    fn arduino_draw_external_mask_raw(
        x: c_int,
        y: c_int,
        bitmap: *const c_uchar,
        mask: *const c_uchar,
        frame: c_uchar,
        mask_frame: c_uchar,
    );
    #[link_name = "arduino_draw_plus_mask"]
    fn arduino_draw_plus_mask_raw(x: c_int, y: c_int, bitmap: *const c_uchar, frame: c_uchar);
    #[link_name = "arduino_draw_erase"]
    fn arduino_draw_erase_raw(x: c_int, y: c_int, bitmap: *const c_uchar, frame: c_uchar);
    #[link_name = "arduino_draw_self_masked"]
    fn arduino_draw_self_masked_raw(x: c_int, y: c_int, bitmap: *const c_uchar, frame: c_uchar);

}
/// Draw a sprite by replacing the existing content completely.
///
/// ### Parameters
///
/// x,y	The coordinates of the top left pixel location.
///
/// bitmap	A pointer to the array containing the image frames.
///
/// frame	The frame number of the image to draw.
///
/// A sprite is drawn by overwriting the pixels in the buffer with the data from the specified frame in the array. No masking is done. A bit set to 1 in the frame will set the pixel to 1 in the buffer, and a 0 in the array will set a 0 in the buffer.
///```
/// image  before  after  (# = 1, - = 0)
///
/// -----  -----   -----
/// --#--  -----   --#--
/// ###-##  -----   ##-##
/// --#--  -----   --#--
/// -----  -----   -----
///
/// image  before  after
///
/// -----  #####   -----
/// --#--  #####   --#--
/// ###-##  #####   ##-##
/// --#--  #####   --#--
/// -----  #####   -----
/// ```
pub fn draw_override(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_override_raw(x, y, bitmap, frame) }
}
pub fn draw_external_mask(
    x: i16,
    y: i16,
    bitmap: *const u8,
    mask: *const u8,
    frame: u8,
    mask_frame: u8,
) {
    unsafe { arduino_draw_external_mask_raw(x, y, bitmap, mask, frame, mask_frame) }
}
pub fn draw_plus_mask(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_plus_mask_raw(x, y, bitmap, frame) }
}
pub fn draw_erase(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_erase_raw(x, y, bitmap, frame) }
}
pub fn draw_self_masked(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_self_masked_raw(x, y, bitmap, frame) }
}
