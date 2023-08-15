#pragma once

// Comment out the librarys you dont need.
// This saves memory but be carful if the librarys are not defined
// it will not export the given functions for the rust project.
//
// to save a good amount you can comment out
// the ArduboyTones library if you don't need it.

#define Arduboy2_Library ;
#define ArduboyTones_Library ;
#define EEPROM_Library ;
#define Arduino_Library ;

#if defined(Arduboy2_Library)
#include <Arduboy2.h>
Arduboy2 arduboy;
#include "./library/arduboy/arduboy_export.h"
#include "./library/arduboy/sprites_export.h"
#endif

#if defined(ArduboyTones_Library)
#include <ArduboyTones.h>
#include "./library/arduboy/arduboy_tones_export.h"
#endif

#if defined(Arduino_Library)
#include "./library/arduino/arduino_export.h"
#endif

#if defined(EEPROM_Library)
#include "./library/arduino/eeprom_export.h"
#endif
