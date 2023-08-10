#pragma once

#include <ArduboyTones.h>
#include <Arduboy2.h>
extern ArduboyTones sound;
extern Arduboy2 arduboy;

extern "C"
{
    void arduboy_audio_on();
    void arduboy_audio_off();
    bool arduboy_audio_enabled();
    void arduboy_audio_toggle();
    void arduboy_audio_save_on_off();
    void sound_tone(unsigned int frequency, unsigned long duration);
    void sound_tone2(unsigned int frequency1, unsigned long duration1, unsigned int frequency2, unsigned long duration2);
    void sound_tone3(unsigned int frequency1, unsigned long duration1, unsigned int frequency2, unsigned long duration2, unsigned int frequency3, unsigned long duration3);
    void sound_tones(const uint16_t *tones);
    void sound_no_tone();
    bool sound_playing();
    void sound_tones_in_ram(uint16_t *tones);
    void sound_volume_mode(uint8_t mode);
}