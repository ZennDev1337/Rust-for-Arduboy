#pragma once
#include <Sprites.h>

extern "C"
{
    void arduino_draw_override(int16_t x, int16_t y, const uint8_t *bitmap, uint8_t frame);
}