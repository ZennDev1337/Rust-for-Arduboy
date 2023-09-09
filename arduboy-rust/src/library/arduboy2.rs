//! This is the Module to interact in a save way with the Arduboy2 C++ library.
//!
//! All of the functions are safe wrapped inside the [Arduboy2] struct.
#![allow(dead_code)]
use crate::hardware::buttons::ButtonSet;
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// Position X
    pub x: i16,
    /// Position Y
    pub y: i16,
}

/// This is the struct to interact in a save way with the Arduboy2 C++ library.
pub struct Arduboy2 {}
impl Arduboy2 {
    /// gives you a new instance of the [Arduboy2]
    /// ## Example
    /// ```
    /// const arduboy: Arduboy2 = Arduboy2::new();
    /// ```
    pub const fn new() -> Self {
        Arduboy2 {}
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
    ///- x	The X coordinate of the left start point.
    ///- y	The Y coordinate of the left start point.
    ///- w	The width of the line.
    ///
    ///color	The color of the line (optional; defaults to WHITE).
    pub fn draw_fast_hline(&self, x: i16, y: i16, w: u8, color: Color) {
        unsafe { draw_fast_hline_raw(x, y, w, color as u8) }
    }
    ///Draw a vertical line.
    ///
    ///### Parameters:
    ///
    ///- x	The X coordinate of the left start point.
    ///- y	The Y coordinate of the left start point.
    ///- h	The height of the line.
    ///
    ///color	The color of the line (optional; defaults to WHITE).
    pub fn draw_fast_vline(&self, x: i16, y: i16, h: u8, color: Color) {
        unsafe { draw_fast_vline_raw(x, y, h, color as u8) }
    }
    ///Set a single pixel in the display buffer to the specified color.
    ///
    ///### Parameters
    ///- x	The X coordinate of the pixel.
    ///- y	The Y coordinate of the pixel.
    ///- color	The color of the pixel (optional; defaults to WHITE).
    ///
    ///The single pixel specified location in the display buffer is set to the specified color. The values WHITE or BLACK can be used for the color. If the color parameter isn't included, the pixel will be set to WHITE.
    pub fn draw_pixel(&self, x: i16, y: i16, color: Color) {
        unsafe { draw_pixel_raw(x, y, color as u8) }
    }
    ///Draw a filled-in rectangle of a specified width and height.
    ///
    ///### Parameters
    ///
    ///- x	The X coordinate of the upper left corner.
    ///- y	The Y coordinate of the upper left corner.
    ///- w	The width of the rectangle.
    ///- h	The height of the rectangle.
    ///
    ///color	The color of the pixel (optional; defaults to WHITE).
    pub fn fill_rect(&self, x: i16, y: i16, w: u8, h: u8, color: Color) {
        unsafe { fill_rect_raw(x, y, w, h, color as u8) }
    }
    ///Draw a rectangle of a specified width and height.
    ///
    ///Parameters
    ///-    x	The X coordinate of the upper left corner.
    ///-    y	The Y coordinate of the upper left corner.
    ///-    w	The width of the rectangle.
    ///-    h	The height of the rectangle.
    ///-    color	The color of the pixel (optional; defaults to WHITE).
    pub fn draw_rect(&self, x: i16, y: i16, w: u8, h: u8, color: Color) {
        unsafe { draw_rect_raw(x, y, w, h, color as u8) }
    }
    ///Draw a circle of a given radius.
    ///
    ///Parameters
    ///-    x0	The X coordinate of the circle's center.
    ///-    y0	The Y coordinate of the circle's center.
    ///-    r	The radius of the circle in pixels.
    ///-    color	The circle's color (optional; defaults to WHITE).
    pub fn draw_circle(&self, x: i16, y: i16, r: u8, color: Color) {
        unsafe { draw_circle_raw(x, y, r, color as u8) }
    }
    ///Draw a filled-in circle of a given radius.
    ///
    ///### Parameters
    ///
    ///- x	The X coordinate of the circle's center.
    ///- y	The Y coordinate of the circle's center.
    ///- r	The radius of the circle in pixels.
    ///
    ///color	The circle's color (optional; defaults to WHITE).
    pub fn fill_circle(&self, x: i16, y: i16, r: u8, color: Color) {
        unsafe { fill_circle_raw(x, y, r, color as u8) }
    }
    ///Draw a filled-in rectangle with rounded corners.
    ///
    ///Parameters
    ///-    x	The X coordinate of the left edge.
    ///-    y	The Y coordinate of the top edge.
    ///-    w	The width of the rectangle.
    ///-    h	The height of the rectangle.
    ///-    r	The radius of the semicircles forming the corners.
    ///-    color	The color of the rectangle (optional; defaults to WHITE).
    pub fn fill_round_rect(&self, x: i16, y: i16, w: u8, h: u8, r: u8, color: Color) {
        unsafe { fill_round_rect(x, y, w, h, r, color as u8) }
    }
    ///Draw a rectangle with rounded corners.
    ///
    ///Parameters
    ///-    x	The X coordinate of the left edge.
    ///-    y	The Y coordinate of the top edge.
    ///-    w	The width of the rectangle.
    ///-    h	The height of the rectangle.
    ///-    r	The radius of the semicircles forming the corners.
    ///-    color	The color of the rectangle (optional; defaults to WHITE).
    pub fn draw_round_rect(&self, x: i16, y: i16, w: u8, h: u8, r: u8, color: Color) {
        unsafe { draw_round_rect(x, y, w, h, r, color as u8) }
    }
    ///Draw a triangle given the coordinates of each corner.
    ///
    ///Parameters
    ///-    x0,x1,x2	The X coordinates of the corners.
    ///-    y0,y1,y2	The Y coordinates of the corners.
    ///-    color	The triangle's color (optional; defaults to WHITE).
    ///
    ///A triangle is drawn by specifying each of the three corner locations. The corners can be at any position with respect to the others.
    pub fn draw_triangle(
        &self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: Color,
    ) {
        unsafe { draw_triangle(x0, y0, x1, y1, x2, y2, color as u8) }
    }
    ///Draw a filled-in triangle given the coordinates of each corner.
    ///
    ///Parameters
    ///-    x0,x1,x2	The X coordinates of the corners.
    ///-    y0,y1,y2	The Y coordinates of the corners.
    ///-    color	The triangle's color (optional; defaults to WHITE).
    ///
    ///A triangle is drawn by specifying each of the three corner locations. The corners can be at any position with respect to the others.
    pub fn fill_triangle(
        &self,
        x0: i16,
        y0: i16,
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
        color: Color,
    ) {
        unsafe { fill_triangle(x0, y0, x1, y1, x2, y2, color as u8) }
    }
    ///Returns the state of the given pixel in the screen buffer.
    ///
    ///### Parameters
    ///- x	The X coordinate of the pixel.
    ///- y	The Y coordinate of the pixel.
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
    ///- button	The button to test for. Only one button should be specified.
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
    ///- button	The button to test for. Only one button should be specified.
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
    ///- buttons	A bit mask indicating which buttons to test. (Can be a single button)
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
    ///-   buttons	A bit mask indicating which buttons to test. (Can be a single button)
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
    /// ```
    /// let value: i16 = 42;
    ///
    /// arduboy.print(b"Hello World\n\0"[..]); // Prints "Hello World" and then sets the
    ///                                        // text cursor to the start of the next line
    /// arduboy.print(f!(b"Hello World\n")); // Prints "Hello World" but does not use the 2kb ram
    /// arduboy.print(value); // Prints "42"
    /// arduboy.print("\n\0"); // Sets the text cursor to the start of the next line
    /// arduboy.print("hello world") // Prints normal [&str]
    /// ```
    pub fn print(&self, x: impl Printable) {
        x.print()
    }
    ///Set the location of the text cursor.
    ///
    ///### Parameters
    ///-   x	The X (horizontal) coordinate, in pixels, for the new location of the text cursor.
    ///
    /// -  y	The Y (vertical) coordinate, in pixels, for the new location of the text cursor.
    ///
    ///The location of the text cursor is set the the specified coordinates. The coordinates are in pixels. Since the coordinates can specify any pixel location, the text does not have to be placed on specific rows. As with all drawing functions, location 0, 0 is the top left corner of the display. The cursor location represents the top left corner of the next character written.
    pub fn set_cursor(&self, x: i16, y: i16) {
        unsafe { set_cursor(x, y) }
    }
    ///Set the frame rate used by the frame control functions.
    ///
    ///### Parameters
    ///-   rate	The desired frame rate in frames per second.
    ///
    ///Normally, the frame rate would be set to the desired value once, at the start of the game, but it can be changed at any time to alter the frame update rate.
    pub fn set_frame_rate(&self, rate: u8) {
        unsafe { set_frame_rate(rate) }
    }
    ///Set the text character size.
    ///
    ///### Parameters
    ///-   s	The text size multiplier. Must be 1 or higher.
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
    /// Save the current sound state in EEPROM.
    ///
    ///The current sound state, set by on() or off(), is saved to the reserved system area in EEPROM. This allows the state to carry over between power cycles and after uploading a different sketch.
    ///
    ///Note
    /// EEPROM is limited in the number of times it can be written to. Sketches should not continuously change and then save the state rapidly.
    pub fn audio_save_on_off(&self) {
        unsafe { arduboy_audio_save_on_off() }
    }
    ///Toggle the sound on/off state.
    ///
    ///If the system is configured for sound on, it will be changed to sound off (mute). If sound is off, it will be changed to on. This function sets the sound mode only until the unit is powered off. To save the current mode use saveOnOff().
    pub fn audio_toggle(&self) {
        unsafe { arduboy_audio_toggle() }
    }
    /// Combines the use function of `audio_on()` and `audio_save_on_off()`
    pub fn audio_on_and_save(&self) {
        unsafe {
            arduboy_audio_on();
            arduboy_audio_save_on_off()
        }
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
    ///- inverse	true will invert the display. false will set the display to no-inverted.
    ///
    ///Calling this function with a value of true will set the display to inverted mode. A pixel with a value of 0 will be on and a pixel set to 1 will be off.
    ///
    ///Once in inverted mode, the display will remain this way until it is set back to non-inverted mode by calling this function with false.
    pub fn invert(&self, inverse: bool) {
        unsafe { arduboy_invert(inverse) }
    }
    ///Test if a point falls within a rectangle.
    ///
    ///Parameters
    ///-    point	A structure describing the location of the point.
    ///-    rect	A structure describing the location and size of the rectangle.
    ///
    ///Returns
    ///    true if the specified point is within the specified rectangle.
    ///
    ///This function is intended to detemine if an object, whose boundaries are defined by the given rectangle, is in contact with the given point.
    pub fn collide_point(&self, point: Point, rect: Rect) -> bool {
        point.x >= rect.x
            && point.x < rect.x + rect.width as i16
            && point.y >= rect.y
            && point.y < rect.y + rect.height as i16
    }
    ///Test if a rectangle is intersecting with another rectangle.
    ///
    ///Parameters
    /// -   rect1,rect2	Structures describing the size and locations of the rectangles.
    ///
    ///Returns
    ///    true if the first rectangle is intersecting the second.
    ///
    ///This function is intended to detemine if an object, whose boundaries are defined by the given rectangle, is in contact with another rectangular object.
    pub fn collide_rect(&self, rect1: Rect, rect2: Rect) -> bool {
        !(rect2.x >= rect1.x + rect1.width as i16
            || rect2.x + rect2.width as i16 <= rect1.x
            || rect2.y >= rect1.y + rect1.height as i16
            || rect2.y + rect2.height as i16 <= rect1.y)
    }
    /// Set one of the RGB LEDs digitally, to either fully on or fully off.
    ///
    /// Parameters
    /// -    color	The name of the LED to set. The value given should be one of RED_LED, GREEN_LED or BLUE_LED.
    /// -    val	Indicates whether to turn the specified LED on or off. The value given should be RGB_ON or RGB_OFF.
    ///
    /// This 2 parameter version of the function will set a single LED within the RGB LED either fully on or fully off. See the description of the 3 parameter version of this function for more details on the RGB LED.
    pub fn digital_write_rgb_single(&self, color: u8, val: u8) {
        unsafe { digital_write_rgb_single(color, val) }
    }
    ///Set the RGB LEDs digitally, to either fully on or fully off.
    ///
    ///Parameters
    ///-    red,green,blue	Use value RGB_ON or RGB_OFF to set each LED.
    ///
    ///The RGB LED is actually individual red, green and blue LEDs placed very close together in a single package. This 3 parameter version of the function will set each LED either on or off, to set the RGB LED to 7 different colors at their highest brightness or turn it off.
    ///```text
    /// The colors are as follows:
    /// RED LED    GREEN LED    BLUE LED    COLOR
    /// RGB_OFF    RGB_OFF      RGB_OFF     OFF
    /// RGB_OFF    RGB_OFF      RGB_ON      Blue
    /// RGB_OFF    RGB_ON       RGB_OFF     Green
    /// RGB_OFF    RGB_ON       RGB_ON      Cyan
    /// RGB_ON     RGB_OFF      RGB_OFF     Red
    /// RGB_ON     RGB_OFF      RGB_ON      Magenta
    /// RGB_ON     RGB_ON       RGB_OFF     Yellow
    /// RGB_ON     RGB_ON       RGB_ON      White
    /// ```
    pub fn digital_write_rgb(&self, red: u8, green: u8, blue: u8) {
        unsafe { digital_write_rgb(red, green, blue) }
    }

    pub fn set_rgb_led(&self, red: u8, green: u8, blue: u8) {
        unsafe { set_rgb_led(red, green, blue) }
    }
    ///Indicate if the specified number of frames has elapsed.
    ///
    ///Parameters
    ///-    frames	The desired number of elapsed frames.
    ///
    ///Returns
    ///    true if the specified number of frames has elapsed.
    ///
    ///This function should be called with the same value each time for a given event. It will return true if the given number of frames has elapsed since the previous frame in which it returned true.
    ///
    ///## Example
    ///If you wanted to fire a shot every 5 frames while the A button is being held down:
    /// ```
    /// if arduboy.everyXFrames(5) {
    ///     if arduboy.pressed(A_BUTTON) {
    ///         fireShot();
    ///     }
    /// }
    /// ```
    pub fn every_x_frames(&self, frames: u8) -> bool {
        unsafe { every_x_frames(frames) }
    }
    ///Flip the display vertically or set it back to normal.
    ///
    ///Parameters
    ///-    flipped	true will set vertical flip mode. false will set normal vertical orientation.
    ///
    ///Calling this function with a value of true will cause the Y coordinate to start at the bottom edge of the display instead of the top, effectively flipping the display vertically.
    ///
    ///Once in vertical flip mode, it will remain this way until normal vertical mode is set by calling this function with a value of false.
    pub fn flip_vertical(&self, flipped: bool) {
        unsafe { flip_vertical(flipped) }
    }
    ///Flip the display horizontally or set it back to normal.
    ///
    ///Parameters
    /// -   flipped	true will set horizontal flip mode. false will set normal horizontal orientation.
    ///
    ///Calling this function with a value of true will cause the X coordinate to start at the left edge of the display instead of the right, effectively flipping the display horizontally.
    ///
    ///Once in horizontal flip mode, it will remain this way until normal horizontal mode is set by calling this function with a value of false.
    pub fn flip_horizontal(&self, flipped: bool) {
        unsafe { flip_horizontal(flipped) }
    }
    ///Set the text foreground color.
    ///
    ///Parameters
    ///-    color	The color to be used for following text. The values WHITE or BLACK should be used.
    pub fn set_text_color(&self, color: Color) {
        unsafe { set_text_color(color as u8) }
    }
    ///Set the text background color.
    ///
    ///Parameters
    ///-    color	The background color to be used for following text. The values WHITE or BLACK should be used.
    ///
    ///The background pixels of following characters will be set to the specified color.
    ///
    ///However, if the background color is set to be the same as the text color, the background will be transparent. Only the foreground pixels will be drawn. The background pixels will remain as they were before the character was drawn.
    pub fn set_text_background_color(&self, color: Color) {
        unsafe { set_text_background_color(color as u8) }
    }
    ///Set the X coordinate of the text cursor location.
    ///
    ///Parameters
    /// -   x	The X (horizontal) coordinate, in pixels, for the new location of the text cursor.
    ///
    ///The X coordinate for the location of the text cursor is set to the specified value, leaving the Y coordinate unchanged. For more details about the text cursor, see the setCursor() function.
    pub fn set_cursor_x(&self, x: i16) {
        unsafe { set_cursor_x(x) }
    }
    ///Set the Y coordinate of the text cursor location.
    ///
    ///Parameters
    ///-    y	The Y (vertical) coordinate, in pixels, for the new location of the text cursor.
    ///
    ///The Y coordinate for the location of the text cursor is set to the specified value, leaving the X coordinate unchanged. For more details about the text cursor, see the setCursor() function.
    pub fn set_cursor_y(&self, y: i16) {
        unsafe { set_cursor_y(y) }
    }
    ///Set or disable text wrap mode.
    ///
    ///Parameters
    /// -   w	true enables text wrap mode. false disables it.
    ///
    ///Text wrap mode is enabled by specifying true. In wrap mode, if a character to be drawn would end up partially or fully past the right edge of the screen (based on the current text size), it will be placed at the start of the next line. The text cursor will be adjusted accordingly.
    ///
    ///If wrap mode is disabled, characters will always be written at the current text cursor position. A character near the right edge of the screen may only be partially displayed and characters drawn at a position past the right edge of the screen will remain off screen.
    pub fn set_text_wrap(&self, w: bool) {
        unsafe { set_text_wrap(w) }
    }
    ///Idle the CPU to save power.
    ///
    ///This puts the CPU in idle sleep mode. You should call this as often as you can for the best power savings. The timer 0 overflow interrupt will wake up the chip every 1ms, so even at 60 FPS a well written app should be able to sleep maybe half the time in between rendering it's own frames.
    pub fn idle(&self) {
        unsafe { idle() }
    }
}

extern "C" {

    #[link_name = "arduboy_begin"]
    fn begin();

    #[link_name = "arduboy_clear"]
    fn clear();

    #[link_name = "arduboy_display"]
    fn display();

    #[link_name = "arduboy_display_and_clear_buffer"]
    fn display_and_clear_buffer();

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

    #[link_name = "arduboy_fill_round_rect"]
    fn fill_round_rect(x: i16, y: i16, w: u8, h: u8, r: u8, color: u8);

    #[link_name = "arduboy_draw_round_rect"]
    fn draw_round_rect(x: i16, y: i16, w: u8, h: u8, r: u8, color: u8);

    #[link_name = "arduboy_fill_triangle"]
    fn fill_triangle(x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    #[link_name = "arduboy_draw_triangle"]
    fn draw_triangle(x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    #[link_name = "arduboy_get_pixel"]
    fn get_pixel_raw(x: u8, y: u8) -> u8;
    #[doc(hidden)]
    #[link_name = "arduboy_init_random_seed"]
    fn init_random_seed();
    #[doc(hidden)]
    #[link_name = "arduboy_just_pressed"]
    pub fn just_pressed(button: u8) -> bool;
    #[doc(hidden)]
    #[link_name = "arduboy_just_released"]
    pub fn just_released(button: u8) -> bool;
    #[doc(hidden)]
    #[link_name = "arduboy_not_pressed"]
    pub fn not_pressed(button: u8) -> bool;

    #[link_name = "arduboy_next_frame"]
    fn next_frame() -> bool;

    #[link_name = "arduboy_poll_buttons"]
    fn poll_buttons();
    #[doc(hidden)]
    #[link_name = "arduboy_pressed"]
    pub fn pressed(buttons: u8) -> bool;
    #[doc(hidden)]
    #[link_name = "arduboy_print_chars"]
    pub fn print_chars(cstr: *const c_char);
    #[doc(hidden)]
    #[link_name = "arduboy_print_chars_progmem"]
    pub fn print_chars_progmem(pstring: *const c_char);

    // #[link_name = "arduboy_print_char"]
    // fn print_char(c: c_char) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduboy_print_int"]
    pub fn print_int(n: c_int, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduboy_print_long"]
    pub fn print_long(n: c_long, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduboy_print_unsigned_char"]
    pub fn print_unsigned_char(n: c_uchar, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduboy_print_unsigned_int"]
    pub fn print_unsigned_int(n: c_uint, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduboy_print_unsigned_long"]
    pub fn print_unsigned_long(n: c_ulong, base: c_int) -> c_size_t;

    #[link_name = "arduboy_set_cursor"]
    fn set_cursor(x: i16, y: i16);

    #[link_name = "arduboy_set_frame_rate"]
    fn set_frame_rate(rate: u8);

    #[link_name = "arduboy_set_text_size"]
    fn set_text_size(size: u8);

    #[link_name = "arduboy_audio_on"]
    fn arduboy_audio_on();

    #[link_name = "arduboy_audio_off"]
    fn arduboy_audio_off();

    #[link_name = "arduboy_audio_save_on_off"]
    fn arduboy_audio_save_on_off();

    #[link_name = "arduboy_audio_toggle"]
    fn arduboy_audio_toggle();

    #[link_name = "arduboy_audio_enabled"]
    fn arduboy_audio_enabled() -> bool;

    #[link_name = "arduboy_invert"]
    fn arduboy_invert(inverse: bool);

    #[link_name = "arduboy_every_x_frames"]
    fn every_x_frames(frames: u8) -> bool;

    #[link_name = "arduboy_flip_horizontal"]
    fn flip_horizontal(flipped: bool);

    #[link_name = "arduboy_flip_vertical"]
    fn flip_vertical(flipped: bool);

    #[link_name = "arduboy_set_text_color"]
    fn set_text_color(color: u8);

    #[link_name = "arduboy_set_text_background_color"]
    fn set_text_background_color(color: u8);

    #[link_name = "arduboy_set_cursor_x"]
    fn set_cursor_x(x: i16);
    #[link_name = "arduboy_set_cursor_y"]
    fn set_cursor_y(y: i16);

    #[link_name = "arduboy_set_text_wrap"]
    fn set_text_wrap(w: bool);

    #[link_name = "arduboy_idle"]
    fn idle();

    #[link_name = "arduboy_digital_write_rgb_single"]
    fn digital_write_rgb_single(color: c_uchar, val: c_uchar);

    #[link_name = "arduboy_digital_write_rgb"]
    fn digital_write_rgb(red: c_uchar, green: c_uchar, blue: c_uchar);
    
    #[link_name = "arduboy_set_rgb_led"]
    fn set_rgb_led(red: c_uchar, green: c_uchar, blue: c_uchar);
}
