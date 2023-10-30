use core::ffi::{c_char, c_int, c_long, c_size_t, c_uchar, c_uint, c_ulong};

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
    pub fn draw_fast_hline_raw(x: i16, y: i16, w: u8, color: u8);

    #[link_name = "arduboy_draw_fast_vline"]
    pub fn draw_fast_vline_raw(x: i16, y: i16, h: u8, color: u8);

    #[link_name = "arduboy_draw_pixel"]
    pub fn draw_pixel_raw(x: i16, y: i16, color: u8);

    #[link_name = "arduboy_draw_circle"]
    pub fn draw_circle_raw(x: i16, y: i16, r: u8, color: u8);

    #[link_name = "arduboy_draw_rect"]
    pub fn draw_rect_raw(x: i16, y: i16, w: u8, h: u8, color: u8);

    #[link_name = "arduboy_fill_circle"]
    pub fn fill_circle_raw(x: i16, y: i16, r: u8, color: u8);

    #[link_name = "arduboy_fill_rect"]
    pub fn fill_rect_raw(x: i16, y: i16, w: u8, h: u8, color: u8);

    #[link_name = "arduboy_fill_round_rect"]
    pub fn fill_round_rect(x: i16, y: i16, w: u8, h: u8, r: u8, color: u8);

    #[link_name = "arduboy_draw_round_rect"]
    pub fn draw_round_rect(x: i16, y: i16, w: u8, h: u8, r: u8, color: u8);

    #[link_name = "arduboy_fill_triangle"]
    pub fn fill_triangle(x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    #[link_name = "arduboy_draw_triangle"]
    pub fn draw_triangle(x0: i16, y0: i16, x1: i16, y1: i16, x2: i16, y2: i16, color: u8);

    #[link_name = "arduboy_get_pixel"]
    pub fn get_pixel_raw(x: u8, y: u8) -> u8;

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
    #[allow(dead_code)]
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
    pub fn arduboy_audio_on();

    #[link_name = "arduboy_audio_off"]
    pub fn arduboy_audio_off();

    #[link_name = "arduboy_audio_save_on_off"]
    pub fn arduboy_audio_save_on_off();

    #[link_name = "arduboy_audio_toggle"]
    pub fn arduboy_audio_toggle();

    #[link_name = "arduboy_audio_enabled"]
    pub fn arduboy_audio_enabled() -> bool;

    #[link_name = "arduboy_invert"]
    pub fn arduboy_invert(inverse: bool);

    #[link_name = "arduboy_every_x_frames"]
    pub fn every_x_frames(frames: u8) -> bool;

    #[link_name = "arduboy_flip_horizontal"]
    pub fn flip_horizontal(flipped: bool);

    #[link_name = "arduboy_flip_vertical"]
    pub fn flip_vertical(flipped: bool);

    #[link_name = "arduboy_set_text_color"]
    pub fn set_text_color(color: u8);

    #[link_name = "arduboy_set_text_background_color"]
    pub fn set_text_background_color(color: u8);

    #[link_name = "arduboy_set_cursor_x"]
    pub fn set_cursor_x(x: i16);

    #[link_name = "arduboy_set_cursor_y"]
    pub fn set_cursor_y(y: i16);

    #[link_name = "arduboy_set_text_wrap"]
    pub fn set_text_wrap(w: bool);

    #[link_name = "arduboy_idle"]
    pub fn idle();

    #[link_name = "arduboy_digital_write_rgb_single"]
    pub fn digital_write_rgb_single(color: c_uchar, val: c_uchar);

    #[link_name = "arduboy_digital_write_rgb"]
    pub fn digital_write_rgb(red: c_uchar, green: c_uchar, blue: c_uchar);

    #[link_name = "arduboy_set_rgb_led_single"]
    pub fn set_rgb_led_single(color: c_uchar, val: c_uchar);

    #[link_name = "arduboy_set_rgb_led"]
    pub fn set_rgb_led(red: c_uchar, green: c_uchar, blue: c_uchar);

    #[link_name = "arduboy_buttons_state"]
    pub fn arduboy_buttons_state() -> u8;

    #[link_name = "arduboy_exit_to_bootloader"]
    pub fn arduboy_exit_to_bootloader();
}
