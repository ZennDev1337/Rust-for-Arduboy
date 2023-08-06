#pragma once

#include <EEPROM.h>

extern "C"
{
    uint8_t arduboy_eeprom_read(int idx);
    void arduboy_eeprom_update(int idx, uint8_t val);
    void arduboy_eeprom_write(int idx, uint8_t val);
    uint8_t arduboy_eeprom_get(int idx);
    void arduboy_eeprom_put(int idx, uint8_t val);
}
