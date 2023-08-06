#include "eeprom.h"

int eeprom;

uint8_t arduboy_eeprom_read(int idx)
{
    eeprom = EEPROM.read(idx);
    return eeprom;
}
void arduboy_eeprom_update(int idx, uint8_t val)
{
    EEPROM.update(idx, val);
}
void arduboy_eeprom_write(int idx, uint8_t val)
{
    EEPROM.write(idx, val);
}
uint8_t arduboy_eeprom_get(int idx)
{
    EEPROM.get(idx, eeprom);
    return eeprom;
}
void arduboy_eeprom_put(int idx, uint8_t val)
{
    EEPROM.put(idx, val);
}