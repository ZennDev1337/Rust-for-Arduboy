#include "arduboy_tones.h"

void arduboy_audio_on()
{
    arduboy.audio.on();
}
void arduboy_audio_off()
{
    arduboy.audio.off();
}
void arduboy_audio_toggle()
{
    arduboy.audio.toggle();
}
void arduboy_audio_save_on_off()
{
    arduboy.audio.saveOnOff();
}
bool arduboy_audio_enabled()
{
    return arduboy.audio.enabled();
}
void sound_tone(unsigned int frequency, unsigned long duration)
{
    sound.tone(frequency, duration);
}
void sound_tone2(unsigned int frequency1, unsigned long duration1, unsigned int frequency2, unsigned long duration2)
{
    sound.tone(frequency1, duration1, frequency2, duration2);
}
void sound_tone3(unsigned int frequency1, unsigned long duration1, unsigned int frequency2, unsigned long duration2, unsigned int frequency3, unsigned long duration3)
{
    sound.tone(frequency1, duration1, frequency2, duration2, frequency3, duration3);
}
void sound_tones(const uint16_t *tones)
{
    sound.tones(tones);
}
void sound_no_tone()
{
    sound.noTone();
}
bool sound_playing()
{
    sound.playing();
}
void sound_tones_in_ram(uint16_t *tones)
{
    sound.tonesInRAM(tones);
}
void sound_volume_mode(uint8_t mode)
{
    sound.volumeMode(mode);
}
