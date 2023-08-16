#pragma once
#include <avr/eeprom.h>

extern "C"
{
    uint8_t arduboy_eeprom_read(int idx)
    {
        return eeprom_read_byte(reinterpret_cast<const uint8_t *>(idx));
    }
    void arduboy_eeprom_update(int idx, uint8_t val)
    {
        eeprom_update_byte(reinterpret_cast<uint8_t *>(idx), val);
    }
    void arduboy_eeprom_write(int idx, uint8_t val)
    {
        eeprom_write_byte(reinterpret_cast<uint8_t *>(idx), val);
    }
    void arduboy_eeprom_get(uint16_t address, uint8_t *object, size_t size)
    {
        eeprom_read_block(object, reinterpret_cast<const uint8_t *>(address), size);
    }
    void arduboy_eeprom_put(uint16_t address, const uint8_t *object, size_t size)
    {
        eeprom_update_block(object, reinterpret_cast<uint8_t *>(address), size);
    }
}
