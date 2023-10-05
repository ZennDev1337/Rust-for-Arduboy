#pragma once

/**** FX data header generated by fxdata-build.py tool version 1.15 ****/

using uint24_t = __uint24;

// Initialize FX hardware using  FX::begin(FX_DATA_PAGE); in the setup() function.

constexpr uint16_t FX_DATA_PAGE  = 0xffc9;
constexpr uint24_t FX_DATA_BYTES = 13937;

constexpr uint24_t arduboyFont = 0x000000;
constexpr uint16_t arduboyFontWidth  = 6;
constexpr uint16_t arduboyFontHeight = 8;
constexpr uint16_t  arduboyFontFrames = 256;

constexpr uint24_t maskedFont = 0x000604;
constexpr uint16_t maskedFontWidth  = 16;
constexpr uint16_t maskedFontHeight = 24;
constexpr uint8_t  maskedFontFrames = 128;

constexpr uint24_t helloWorld = 0x003608;
