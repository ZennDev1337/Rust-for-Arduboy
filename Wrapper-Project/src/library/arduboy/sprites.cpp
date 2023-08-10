#include "sprites.h"

void arduino_draw_override(int16_t x, int16_t y, const uint8_t *bitmap, uint8_t frame)
{
    Sprites::drawOverwrite(x, y, bitmap, frame);
}
void arduino_draw_external_mask(int16_t x, int16_t y, const uint8_t *bitmap,
                                const uint8_t *mask, uint8_t frame, uint8_t mask_frame)
{
    Sprites::drawExternalMask(x, y, bitmap, mask, frame, mask_frame);
}
void arduino_draw_plus_mask(int16_t x, int16_t y, const uint8_t *bitmap, uint8_t frame)
{
    Sprites::drawPlusMask(x, y, bitmap, frame);
}
void arduino_draw_erase(int16_t x, int16_t y, const uint8_t *bitmap, uint8_t frame)
{
    Sprites::drawErase(x, y, bitmap, frame);
}
void arduino_draw_self_masked(int16_t x, int16_t y, const uint8_t *bitmap, uint8_t frame)
{
    Sprites::drawSelfMasked(x, y, bitmap, frame);
}