//! This is the Module to interact in a save way with the Arduboy2 C++ library.
//!
//! Ignore the functions here you only need to import the prelude here you will find all sorts of unsafe functions
//! All of them are safe wrapped inside the struct.
#![allow(dead_code)]
use crate::prelude::ButtonSet;
use crate::print::Printable;
use core::ffi::{c_char, c_int, c_long, c_size_t, c_uchar, c_uint, c_ulong};
use core::mem;
use core::ops::Not;
/// The standard font size of the arduboy
///
/// this is to calculate with it.
pub const FONT_SIZE: u8 = 6;
/// The standard width of the arduboy
///
/// this is to calculate with it.
pub const WIDTH: u8 = 128;
/// The standard height of the arduboy
///
/// this is to calculate with it.
pub const HEIGHT: u8 = 64;

/// This item is to chose between Black or White
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Color {
    /// Led is off
    Black,
    /// Led is on
    White,
}
impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
/// This struct is used by a few Arduboy functions.
pub struct Rect {
    /// Position X
    pub x: i16,
    /// Position Y
    pub y: i16,
    /// Rect width
    pub width: u8,
    /// Rect height
    pub height: u8,
}
/// This struct is used by a few Arduboy functions.
pub struct Point {
    /// Position X
    pub x: i16,
    /// Position Y
    pub y: i16,
}

/// This is the struct to interact in a save way with the Arduboy2 C++ library.
pub struct Arduboy {}
impl Arduboy {
    pub fn new() -> Self {
        Arduboy {}
    }
    /// Initialize the hardware, display the boot logo, provide boot utilities, etc.
    /// This function should be called once near the start of the sketch, usually in setup(), before using any other functions in this class. It initializes the display, displays the boot logo, provides "flashlight" and system control features and initializes audio control.
    pub fn begin(&self) {
        unsafe { begin() }
    }
    /// Clear the display buffer and set the text cursor to location 0, 0.
    pub fn clear(&self) {
        unsafe { clear() }
    }
    /// Copy the contents of the display buffer to the display.
    /// The contents of the display buffer in RAM are copied to the display and will appear on the screen.
    pub fn display(&self) {
        unsafe { display() }
    }
    ///Copy the contents of the display buffer to the display. The display buffer will be cleared to zero.
    ///
    ///Operation is the same as calling display() without parameters except additionally the display buffer will be cleared.
    pub fn display_and_clear_buffer(&self) {
        unsafe { display_and_clear_buffer() }
    }
    ///Draw a horizontal line.
    ///
    ///### Parameters:
    ///
    ///x	The X coordinate of the left start point.
    ///
    ///y	The Y coordinate of the left start point.
    ///
    ///w	The width of the line.
    ///
    ///color	The color of the line (optional; defaults to WHITE).
    pub fn draw_fast_hline(&self, x: i16, y: i16, w: u8, color: Color) {
        unsafe { draw_fast_hline_raw(x, y, w, color as u8) }
    }
    ///Draw a vertical line.
    ///
    ///### Parameters:
    ///
    ///x	The X coordinate of the left start point.
    ///
    ///y	The Y coordinate of the left start point.
    ///
    ///h	The height of the line.
    ///
    ///color	The color of the line (optional; defaults to WHITE).
    pub fn draw_fast_vline(&self, x: i16, y: i16, h: u8, color: Color) {
        unsafe { draw_fast_vline_raw(x, y, h, color as u8) }
    }
    ///Set a single pixel in the display buffer to the specified color.
    ///
    ///### Parameters
    ///
    ///x	The X coordinate of the pixel.
    ///
    ///y	The Y coordinate of the pixel.
    ///
    ///color	The color of the pixel (optional; defaults to WHITE).
    ///
    ///The single pixel specified location in the display buffer is set to the specified color. The values WHITE or BLACK can be used for the color. If the color parameter isn't included, the pixel will be set to WHITE.
    pub fn draw_pixel(&self, x: i16, y: i16, color: Color) {
        unsafe { draw_pixel_raw(x, y, color as u8) }
    }
    ///Draw a filled-in rectangle of a specified width and height.
    ///
    ///### Parameters
    ///
    ///x	The X coordinate of the upper left corner.
    ///
    ///y	The Y coordinate of the upper left corner.
    ///
    ///w	The width of the rectangle.
    ///
    ///h	The height of the rectangle.
    ///
    ///color	The color of the pixel (optional; defaults to WHITE).
    pub fn fill_rect(&self, x: i16, y: i16, w: u8, h: u8, color: Color) {
        unsafe { fill_rect_raw(x, y, w, h, color as u8) }
    }
    pub fn draw_rect(&self, x: i16, y: i16, w: u8, h: u8, color: Color) {
        unsafe { draw_rect_raw(x, y, w, h, color as u8) }
    }
    pub fn draw_circle(&self, x: i16, y: i16, r: u8, color: Color) {
        unsafe { draw_circle_raw(x, y, r, color as u8) }
    }
    ///Draw a filled-in circle of a given radius.
    ///
    ///### Parameters
    ///
    ///x0	The X coordinate of the circle's center.
    ///
    ///y0	The Y coordinate of the circle's center.
    ///
    ///r	The radius of the circle in pixels.
    ///
    ///color	The circle's color (optional; defaults to WHITE).
    pub fn fill_circle(&self, x: i16, y: i16, r: u8, color: Color) {
        unsafe { fill_circle_raw(x, y, r, color as u8) }
    }
    ///Returns the state of the given pixel in the screen buffer.
    ///
    ///### Parameters
    ///x	The X coordinate of the pixel.
    ///
    ///y	The Y coordinate of the pixel.
    ///
    ///### Returns
    ///WHITE if the pixel is on or BLACK if the pixel is off.
    pub fn get_pixel(&self, x: u8, y: u8) -> Color {
        unsafe { mem::transmute::<u8, Color>(get_pixel_raw(x, y)) }
    }
    /// Seed the random number generator with a random value.
    ///
    /// The Arduino pseudorandom number generator is seeded with the random value returned from a call to generateRandomSeed().
    pub fn init_random_seed(&self) {
        unsafe { init_random_seed() }
    }
    ///Check if a button has just been pressed.
    ///
    ///### Parameters
    ///button	The button to test for. Only one button should be specified.
    ///
    ///### Returns
    ///true if the specified button has just been pressed.
    ///
    ///Return true if the given button was pressed between the latest call to pollButtons() and previous call to pollButtons(). If the button has been held down over multiple polls, this function will return false.
    ///
    ///There is no need to check for the release of the button since it must have been released for this function to return true when pressed again.
    ///
    ///This function should only be used to test a single button.
    pub fn just_pressed(&self, button: ButtonSet) -> bool {
        unsafe { just_pressed(button.flag_set) }
    }
    ///Check if a button has just been released.
    ///
    ///### Parameters
    ///button	The button to test for. Only one button should be specified.
    ///
    ///### Returns
    ///true if the specified button has just been released.
    ///
    ///Return true if the given button was released between the latest call to pollButtons() and previous call to pollButtons(). If the button has been held down over multiple polls, this function will return false.
    ///
    ///There is no need to check for the released of the button since it must have been pressed for this function to return true when pressed again.
    ///
    ///This function should only be used to test a single button.
    pub fn just_released(&self, button: ButtonSet) -> bool {
        unsafe { just_released(button.flag_set) }
    }
    ///Test if the specified buttons are not pressed.
    ///
    ///### Parameters
    ///
    ///buttons	A bit mask indicating which buttons to test. (Can be a single button)
    ///
    ///### Returns
    ///
    /// True if all buttons in the provided mask are currently released.
    ///
    ///Read the state of the buttons and return true if all the buttons in the specified mask are currently released.
    pub fn not_pressed(&self, button: ButtonSet) -> bool {
        unsafe { not_pressed(button.flag_set) }
    }
    ///Indicate that it's time to render the next frame.
    ///
    ///### Returns
    ///true if it's time for the next frame.
    ///
    ///When this function returns true, the amount of time has elapsed to display the next frame, as specified by setFrameRate() or setFrameDuration().
    ///
    ///This function will normally be called at the start of the rendering loop which would wait for true to be returned before rendering and displaying the next frame.
    pub fn next_frame(&self) -> bool {
        unsafe { next_frame() }
    }
    ///Poll the buttons and track their state over time.
    ///
    ///Read and save the current state of the buttons and also keep track of the button state when this function was previously called. These states are used by the justPressed() and justReleased() functions to determine if a button has changed state between now and the previous call to pollButtons().
    ///
    ///This function should be called once at the start of each new frame.
    ///
    ///The justPressed() and justReleased() functions rely on this function.
    pub fn poll_buttons(&self) {
        unsafe { poll_buttons() }
    }
    ///Test if the all of the specified buttons are pressed.
    ///
    ///### Parameters
    ///   buttons	A bit mask indicating which buttons to test. (Can be a single button)
    ///
    ///### Returns
    ///   true if all buttons in the provided mask are currently pressed.
    ///
    ///Read the state of the buttons and return true if all of the buttons in the specified mask are being pressed.
    pub fn pressed(&self, button: ButtonSet) -> bool {
        unsafe { pressed(button.flag_set) }
    }
    ///The Arduino Print class is available for writing text to the screen buffer.
    ///
    ///For an Arduboy2 class object, functions provided by the Arduino Print class can be used to write text to the screen buffer, in the same manner as the Arduino Serial.print(), etc., functions.
    ///
    ///Print will use the write() function to actually draw each character in the screen buffer, using the library's font5x7 font. Two character values are handled specially:
    ///
    ///- ASCII newline/line feed (\n, 0x0A, inverse white circle). This will move the text cursor position to the start of the next line, based on the current text size.
    ///- ASCII carriage return (\r, 0x0D, musical eighth note). This character will be ignored.
    ///
    ///
    ///Example
    /// ```text
    /// let value:i16 = 42;
    ///
    /// arduboy.println("Hello World\0"); // Prints "Hello World" and then sets the
    ///                                   // text cursor to the start of the next line
    /// arduboy.print(value);             // Prints "42"
    /// arduboy.print('\n\0');            // Sets the text cursor to the start of the next line
    /// arduboy.print(78, HEX);           // Prints "4E" (78 in hexadecimal)
    /// arduboy.print("\x03\xEA");        // Prints a heart symbol and a Greek uppercase omega
    /// ```
    pub fn print(&self, x: impl Printable) {
        x.print()
    }
    ///Set the location of the text cursor.
    ///
    ///### Parameters
    ///   x	The X (horizontal) coordinate, in pixels, for the new location of the text cursor.
    ///
    ///   y	The Y (vertical) coordinate, in pixels, for the new location of the text cursor.
    ///
    ///The location of the text cursor is set the the specified coordinates. The coordinates are in pixels. Since the coordinates can specify any pixel location, the text does not have to be placed on specific rows. As with all drawing functions, location 0, 0 is the top left corner of the display. The cursor location represents the top left corner of the next character written.
    pub fn set_cursor(&self, x: i16, y: i16) {
        unsafe { set_cursor(x, y) }
    }
    ///Set the frame rate used by the frame control functions.
    ///
    ///### Parameters
    ///   rate	The desired frame rate in frames per second.
    ///
    ///Normally, the frame rate would be set to the desired value once, at the start of the game, but it can be changed at any time to alter the frame update rate.
    pub fn set_frame_rate(&self, rate: u8) {
        unsafe { set_frame_rate(rate) }
    }
    ///Set the text character size.
    ///
    ///### Parameters
    ///   s	The text size multiplier. Must be 1 or higher.
    ///
    ///Setting a text size of 1 will result in standard size characters with one pixel for each bit in the bitmap for a character. The value specified is a multiplier. A value of 2 will double the width and height. A value of 3 will triple the dimensions, etc.
    pub fn set_text_size(&self, size: u8) {
        unsafe { set_text_size(size) }
    }
    ///Turn sound on.
    ///
    ///The system is configured to generate sound. This function sets the sound mode only until the unit is powered off.
    pub fn audio_on(&self) {
        unsafe { arduboy_audio_on() }
    }
    ///Turn sound off (mute).
    ///
    ///The system is configured to not produce sound (mute). This function sets the sound mode only until the unit is powered off.
    pub fn audio_off(&self) {
        unsafe { arduboy_audio_off() }
    }
    ///Get the current sound state.
    ///
    ///### Returns
    ///true if sound is currently enabled (not muted).
    ///
    ///This function should be used by code that actually generates sound. If true is returned, sound can be produced. If false is returned, sound should be muted.
    pub fn audio_enabled(&self) -> bool {
        unsafe { arduboy_audio_enabled() }
    }
    ///Invert the entire display or set it back to normal.
    ///
    ///### Parameters
    ///inverse	true will invert the display. false will set the display to no-inverted.
    ///
    ///Calling this function with a value of true will set the display to inverted mode. A pixel with a value of 0 will be on and a pixel set to 1 will be off.
    ///
    ///Once in inverted mode, the display will remain this way until it is set back to non-inverted mode by calling this function with false.
    pub fn invert(&self, inverse: bool) {
        unsafe { arduboy_invert(inverse) }
    }
    pub fn collide_point(point: Point, rect: Rect) -> bool {
        point.x >= rect.x
            && point.x < rect.x + rect.width as i16
            && point.y >= rect.y
            && point.y < rect.y + rect.height as i16
    }
    pub fn collide_rect(rect1: Rect, rect2: Rect) -> bool {
        !(rect2.x >= rect1.x + rect1.width as i16
            || rect2.x + rect2.width as i16 <= rect1.x
            || rect2.y >= rect1.y + rect1.height as i16
            || rect2.y + rect2.height as i16 <= rect1.y)
    }
}

extern "C" {
    #[link_name = "arduboy_begin"]
    pub fn begin();

    #[link_name = "arduboy_clear"]
    pub fn clear();

    #[link_name = "arduboy_display"]
    pub fn display();

    #[link_name = "arduboy_display_and_clear_buffer"]
    pub fn display_and_clear_buffer();

    #[link_name = "arduboy_draw_fast_hline"]
    fn draw_fast_hline_raw(x: i16, y: i16, w: u8, color: u8);

    #[link_name = "arduboy_draw_fast_vline"]
    fn draw_fast_vline_raw(x: i16, y: i16, h: u8, color: u8);

    #[link_name = "arduboy_draw_pixel"]
    fn draw_pixel_raw(x: i16, y: i16, color: u8);

    #[link_name = "arduboy_draw_circle"]
    fn draw_circle_raw(x: i16, y: i16, r: u8, color: u8);

    #[link_name = "arduboy_draw_rect"]
    fn draw_rect_raw(x: i16, y: i16, w: u8, h: u8, color: u8);

    #[link_name = "arduboy_fill_circle"]
    fn fill_circle_raw(x: i16, y: i16, r: u8, color: u8);

    #[link_name = "arduboy_fill_rect"]
    fn fill_rect_raw(x: i16, y: i16, w: u8, h: u8, color: u8);

    #[link_name = "arduboy_get_pixel"]
    fn get_pixel_raw(x: u8, y: u8) -> u8;

    #[link_name = "arduboy_init_random_seed"]
    pub fn init_random_seed();

    #[link_name = "arduboy_just_pressed"]
    pub fn just_pressed(button: u8) -> bool;

    #[link_name = "arduboy_just_released"]
    pub fn just_released(button: u8) -> bool;

    #[link_name = "arduboy_not_pressed"]
    pub fn not_pressed(button: u8) -> bool;

    #[link_name = "arduboy_next_frame"]
    pub fn next_frame() -> bool;

    #[link_name = "arduboy_poll_buttons"]
    pub fn poll_buttons();

    #[link_name = "arduboy_pressed"]
    pub fn pressed(buttons: u8) -> bool;

    #[link_name = "arduboy_print_chars"]
    pub fn print_chars(cstr: *const c_char);

    #[link_name = "arduboy_print_chars_progmem"]
    pub fn print_chars_progmem(pstring: *const c_char);

    // #[link_name = "arduboy_print_char"]
    // fn print_char(c: c_char) -> c_size_t;

    #[link_name = "arduboy_print_int"]
    pub fn print_int(n: c_int, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_long"]
    pub fn print_long(n: c_long, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_unsigned_char"]
    pub fn print_unsigned_char(n: c_uchar, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_unsigned_int"]
    pub fn print_unsigned_int(n: c_uint, base: c_int) -> c_size_t;

    #[link_name = "arduboy_print_unsigned_long"]
    pub fn print_unsigned_long(n: c_ulong, base: c_int) -> c_size_t;

    #[link_name = "arduboy_set_cursor"]
    pub fn set_cursor(x: i16, y: i16);

    #[link_name = "arduboy_set_frame_rate"]
    pub fn set_frame_rate(rate: u8);

    #[link_name = "arduboy_set_text_size"]
    pub fn set_text_size(size: u8);

    #[link_name = "arduboy_audio_on"]
    fn arduboy_audio_on();

    #[link_name = "arduboy_audio_off"]
    fn arduboy_audio_off();

    #[link_name = "arduboy_audio_enabled"]
    fn arduboy_audio_enabled() -> bool;

    #[link_name = "arduboy_invert"]
    fn arduboy_invert(inverse: bool);
}

pub unsafe fn print(x: impl Printable) {
    x.print();
}

pub unsafe fn draw_fast_hline(x: i16, y: i16, w: u8, color: Color) {
    draw_fast_hline_raw(x, y, w, color as u8);
}

pub unsafe fn draw_fast_vline(x: i16, y: i16, h: u8, color: Color) {
    draw_fast_vline_raw(x, y, h, color as u8);
}

pub unsafe fn draw_pixel(x: i16, y: i16, color: Color) {
    draw_pixel_raw(x, y, color as u8);
}

pub unsafe fn fill_rect(x: i16, y: i16, w: u8, h: u8, color: Color) {
    fill_rect_raw(x, y, w, h, color as u8);
}

pub unsafe fn draw_circle(x: i16, y: i16, r: u8, color: Color) {
    draw_circle_raw(x, y, r, color as u8);
}

pub unsafe fn get_pixel(x: u8, y: u8) -> Color {
    mem::transmute::<u8, Color>(get_pixel_raw(x, y))
}
