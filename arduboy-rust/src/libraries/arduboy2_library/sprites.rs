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
/// - x,y	The coordinates of the top left pixel location.
/// - bitmap	A pointer to the array containing the image frames.
/// - frame	The frame number of the image to draw.
///
/// A sprite is drawn by overwriting the pixels in the buffer with the data from the specified frame in the array. No masking is done. A bit set to 1 in the frame will set the pixel to 1 in the buffer, and a 0 in the array will set a 0 in the buffer.
///```text
/// image  before  after  (# = 1, - = 0)
///
/// -----  -----   -----
/// --#--  -----   --#--
/// ##-##  -----   ##-##
/// --#--  -----   --#--
/// -----  -----   -----
///
/// image  before  after
///
/// -----  #####   -----
/// --#--  #####   --#--
/// ##-##  #####   ##-##
/// --#--  #####   --#--
/// -----  #####   -----
/// ```
pub fn draw_override(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_override_raw(x, y, bitmap, frame) }
}
///Draw a sprite using a separate image and mask array.
///
///Parameters
///-    x,y	The coordinates of the top left pixel location.
///-    bitmap	A pointer to the array containing the image frames.
///-    mask	A pointer to the array containing the mask frames.
///-    frame	The frame number of the image to draw.
///-    mask_frame	The frame number for the mask to use (can be different from the image frame number).
///
///An array containing the image frames, and another array containing corresponding mask frames, are used to draw a sprite.
///
///For the mask array, the width and height are not included but must contain data of the same dimensions as the corresponding image array.
///
///Bits set to 1 in the mask indicate that the pixel will be set to the value of the corresponding image bit. Bits set to 0 in the mask will be left unchanged.
///```text
/// image  mask   before  after  (# = 1, - = 0)
///
/// -----  -###-  -----   -----
/// --#--  #####  -----   --#--
/// ##-##  ##-##  -----   ##-##
/// --#--  #####  -----   --#--
/// -----  -###-  -----   -----
///
/// image  mask   before  after
///
/// -----  -###-  #####   #---#
/// --#--  #####  #####   --#--
/// ##-##  #####  #####   ##-##
/// --#--  #####  #####   --#--
/// -----  -###-  #####   #---#
/// ```
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
///Draw a sprite using an array containing both image and mask values.
///
///Parameters
/// -   x,y	The coordinates of the top left pixel location.
/// -   bitmap	A pointer to the array containing the image/mask frames.
/// -   frame	The frame number of the image to draw.
///
///An array containing combined image and mask data is used to draw a sprite. Bytes are given in pairs with the first byte representing the image pixels and the second byte specifying the corresponding mask. The width given in the array still specifies the image width, so each row of image and mask bytes will be twice the width value.
///
///Bits set to 1 in the mask indicate that the pixel will be set to the value of the corresponding image bit. Bits set to 0 in the mask will be left unchanged.
///
///image  mask   before  after  (# = 1, - = 0)
///```text
/// -----  -###-  -----   -----
/// --#--  #####  -----   --#--
/// ##-##  ##-##  -----   ##-##
/// --#--  #####  -----   --#--
/// -----  -###-  -----   -----
///
/// image  mask   before  after
///
/// -----  -###-  #####   #---#
/// --#--  #####  #####   --#--
/// ##-##  #####  #####   ##-##
/// --#--  #####  #####   --#--
/// -----  -###-  #####   #---#
/// ```
pub fn draw_plus_mask(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_plus_mask_raw(x, y, bitmap, frame) }
}
///"Erase" a sprite.
///
///Parameters
/// -   x,y	The coordinates of the top left pixel location.
/// -   bitmap	A pointer to the array containing the image frames.
/// -   frame	The frame number of the image to erase.
///
///The data from the specified frame in the array is used to erase a sprite. To "erase" a sprite, bits set to 1 in the frame will set the corresponding pixel in the buffer to 0. Frame bits set to 0 will remain unchanged in the buffer.
///```text
/// image  before  after  (# = 1, - = 0)
///
/// -----  -----   -----
/// --#--  -----   -----
/// ##-##  -----   -----
/// --#--  -----   -----
/// -----  -----   -----
///
/// image  before  after
///
/// -----  #####   #####
/// --#--  #####   ##-##
/// ##-##  #####   --#--
/// --#--  #####   ##-##
/// -----  #####   #####
/// ```
pub fn draw_erase(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_erase_raw(x, y, bitmap, frame) }
}
///Draw a sprite using only the bits set to 1.
///
///Parameters
/// -   x,y	The coordinates of the top left pixel location.
/// -   bitmap	A pointer to the array containing the image frames.
/// -   frame	The frame number of the image to draw.
///
///Bits set to 1 in the frame will be used to draw the sprite by setting the corresponding pixel in the buffer to 1. Bits set to 0 in the frame will remain unchanged in the buffer.
///```text
/// image  before  after  (# = 1, - = 0)
///
/// -----  -----   -----
/// --#--  -----   --#--
/// ##-##  -----   ##-##
/// --#--  -----   --#--
/// -----  -----   -----
///
/// image  before  after
///
/// -----  #####   #####  (no change because all pixels were
/// --#--  #####   #####  already white)
/// ##-##  #####   #####
/// --#--  #####   #####
/// -----  #####   #####
/// ```
pub fn draw_self_masked(x: i16, y: i16, bitmap: *const u8, frame: u8) {
    unsafe { arduino_draw_self_masked_raw(x, y, bitmap, frame) }
}
