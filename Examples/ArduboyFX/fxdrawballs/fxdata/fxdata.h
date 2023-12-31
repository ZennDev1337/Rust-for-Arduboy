#pragma once

/**** FX data header generated by fxdata-build.py tool version 1.15 ****/

using uint24_t = __uint24;

// Initialize FX hardware using  FX::begin(FX_DATA_PAGE); in the setup() function.

constexpr uint16_t FX_DATA_PAGE  = 0xfffe;
constexpr uint24_t FX_DATA_BYTES = 392;

constexpr uint24_t FX_DATA_TILES = 0x000000;
constexpr uint16_t FX_DATA_TILES_WIDTH  = 16;
constexpr uint16_t FX_DATA_TILESHEIGHT  = 16;
constexpr uint8_t  FX_DATA_TILES_FRAMES = 2;

constexpr uint24_t FX_DATA_TILEMAP = 0x000044;
constexpr uint24_t FX_DATA_BALLS = 0x000144;
constexpr uint16_t FX_DATA_BALLS_WIDTH  = 16;
constexpr uint16_t FX_DATA_BALLSHEIGHT  = 16;

