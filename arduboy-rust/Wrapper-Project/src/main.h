#pragma once

#include "../../../import_config.h"

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
#if defined(ArdVoice_Library)
#include <ArdVoice.h>
ArdVoice ardvoice;
#include "./library/ardvoice/ardvoice.h"
#endif

#if defined(Arduino_Library)
#include "./library/arduino/arduino_export.h"
#endif

#if defined(EEPROM_Library)
#include "./library/arduino/eeprom_export.h"
#endif

#include "./library/arduino/arduino_serial_export.h"
