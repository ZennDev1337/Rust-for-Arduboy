#pragma once
#include <Arduboy2.h>

extern "C"
{
    void arduboy_begin(void);
    void arduboy_clear(void);
    void arduboy_display(void);
    void arduboy_display_and_clear_buffer(void);
    void arduboy_draw_fast_hline(int16_t x, int16_t y, uint8_t w, uint8_t color);
    void arduboy_draw_fast_vline(int16_t x, int16_t y, uint8_t h, uint8_t color);
    void arduboy_draw_pixel(int16_t x, int16_t y, uint8_t color);
    void arduboy_draw_circle(int16_t x, int16_t y, uint8_t r, uint8_t color);
    void arduboy_fill_circle(int16_t x, int16_t y, uint8_t r, uint8_t color);
    void arduboy_draw_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t color);
    void arduboy_fill_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t color);
    unsigned long arduboy_generate_random_seed();
    uint8_t arduboy_get_pixel(uint8_t x, uint8_t y);
    void arduboy_init_random_seed(void);
    bool arduboy_just_pressed(uint8_t button);
    bool arduboy_just_released(uint8_t button);
    bool arduboy_next_frame(void);
    void arduboy_poll_buttons();
    bool arduboy_pressed(uint8_t buttons);
    void arduboy_print_chars(const char *cstr);
    size_t arduboy_print_char(char c);
    size_t arduboy_print_chars_progmem(const char *);
    size_t arduboy_print_int(int n, int base);
    size_t arduboy_print_long(long n, int base);
    size_t arduboy_print_unsigned_char(unsigned char n, int base);
    size_t arduboy_print_unsigned_int(unsigned int n, int base);
    size_t arduboy_print_unsigned_long(unsigned long n, int base);
    void arduboy_set_cursor(int16_t x, int16_t y);
    void arduboy_set_frame_rate(uint8_t rate);
    bool arduboy_not_pressed(uint8_t button);
    void arduboy_set_text_size(uint8_t s);
    void arduboy_invert(bool inverse);
}