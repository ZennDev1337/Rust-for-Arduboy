#pragma once
#include <Arduboy2.h>
extern Arduboy2 arduboy;
extern "C"
{
    void arduboy_begin(void)
    {
        arduboy.begin();
    }
    void arduboy_clear(void)
    {
        arduboy.clear();
    }
    void arduboy_display(void)
    {
        arduboy.display();
    }
    void arduboy_display_and_clear_buffer(void)
    {
        arduboy.display(CLEAR_BUFFER);
    }
    void arduboy_draw_fast_hline(int16_t x, int16_t y, uint8_t w, uint8_t color)
    {
        arduboy.drawFastHLine(x, y, w, color);
    }
    void arduboy_draw_fast_vline(int16_t x, int16_t y, uint8_t h, uint8_t color)
    {
        arduboy.drawFastVLine(x, y, h, color);
    }
    void arduboy_draw_pixel(int16_t x, int16_t y, uint8_t color)
    {
        arduboy.drawPixel(x, y, color);
    }
    void arduboy_draw_circle(int16_t x, int16_t y, uint8_t r, uint8_t color)
    {
        arduboy.drawCircle(x, y, r, color);
    }
    void arduboy_fill_circle(int16_t x, int16_t y, uint8_t r, uint8_t color)
    {
        arduboy.fillCircle(x, y, r, color);
    }
    void arduboy_fill_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t color)
    {
        arduboy.fillRect(x, y, w, h, color);
    }
    void arduboy_draw_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t color)
    {
        arduboy.drawRect(x, y, w, h, color);
    }
    void arduboy_fill_round_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t r, uint8_t color)
    {
        arduboy.fillRoundRect(x, y, w, h, r, color);
    }
    void arduboy_draw_round_rect(int16_t x, int16_t y, uint8_t w, uint8_t h, uint8_t r, uint8_t color)
    {
        arduboy.drawRoundRect(x, y, w, h, r, color);
    }
    void arduboy_fill_triangle(int16_t x0, int16_t y0, int16_t x1, int16_t y1, int16_t x2, int16_t y2, uint8_t color)
    {
        arduboy.fillTriangle(x0, y0, x1, y1, x2, y2, color);
    }
    void arduboy_draw_triangle(int16_t x0, int16_t y0, int16_t x1, int16_t y1, int16_t x2, int16_t y2, uint8_t color)
    {
        arduboy.drawTriangle(x0, y0, x1, y1, x2, y2, color);
    }
    bool arduboy_every_x_frames(uint8_t frames)
    {
        return arduboy.everyXFrames(frames);
    }
    void arduboy_flip_horizontal(bool flipped)
    {
        arduboy.flipHorizontal(flipped);
    }
    void arduboy_flip_vertical(bool flipped)
    {
        arduboy.flipVertical(flipped);
    }
    void arduboy_set_text_color(uint8_t color)
    {
        arduboy.setTextColor(color);
    }
    void arduboy_set_text_background_color(uint8_t color)
    {
        arduboy.setTextBackground(color);
    }
    void arduboy_set_cursor_x(int16_t x)
    {
        arduboy.setCursorX(x);
    }
    void arduboy_set_cursor_y(int16_t y)
    {
        arduboy.setCursorY(y);
    }
    void arduboy_set_text_wrap(bool w)
    {
        arduboy.setTextWrap(w);
    }
    void arduboy_idle()
    {
        arduboy.idle();
    }

    unsigned long arduboy_generate_random_seed()
    {
        return arduboy.generateRandomSeed();
    }
    uint8_t arduboy_get_pixel(uint8_t x, uint8_t y)
    {
        return arduboy.getPixel(x, y);
    }
    void arduboy_init_random_seed(void)
    {
        arduboy.initRandomSeed();
    }
    bool arduboy_just_pressed(uint8_t button)
    {
        return arduboy.justPressed(button);
    }
    bool arduboy_just_released(uint8_t button)
    {
        return arduboy.justReleased(button);
    }
    bool arduboy_next_frame(void)
    {
        return arduboy.nextFrame();
    }
    void arduboy_poll_buttons()
    {
        arduboy.pollButtons();
    }
    bool arduboy_pressed(uint8_t buttons)
    {
        return arduboy.pressed(buttons);
    }
    void arduboy_print_chars(const char *cstr)
    {
        arduboy.print(cstr);
    }
    size_t arduboy_print_chars_progmem(const char *cstr)
    {
        return arduboy.print(reinterpret_cast<const __FlashStringHelper *>(cstr));
    }
    size_t arduboy_print_char(char c)
    {
        return arduboy.print(c);
    }
    size_t arduboy_print_int(int n, int base)
    {
        return arduboy.print(n, base);
    }
    size_t arduboy_print_long(long n, int base)
    {
        return arduboy.print(n, base);
    }
    size_t arduboy_print_unsigned_char(unsigned char n, int base)
    {
        return arduboy.print(n, base);
    }
    size_t arduboy_print_unsigned_int(unsigned int n, int base)
    {
        return arduboy.print(n, base);
    }
    size_t arduboy_print_unsigned_long(unsigned long n, int base)
    {
        return arduboy.print(n, base);
    }
    void arduboy_set_cursor(int16_t x, int16_t y)
    {
        arduboy.setCursor(x, y);
    }
    void arduboy_set_frame_rate(uint8_t rate)
    {
        arduboy.setFrameRate(rate);
    }
    bool arduboy_not_pressed(uint8_t button)
    {
        arduboy.notPressed(button);
    }
    void arduboy_set_text_size(uint8_t s)
    {
        arduboy.setTextSize(s);
    }
    void arduboy_invert(bool inverse)
    {
        arduboy.invert(inverse);
    }
    void arduboy_digital_write_rgb_single(uint8_t color, uint8_t val)
    {
        arduboy.digitalWriteRGB(color, val);
    }
    void arduboy_digital_write_rgb(uint8_t red, uint8_t green, uint8_t blue)
    {
        arduboy.digitalWriteRGB(red, green, blue);
    }
}