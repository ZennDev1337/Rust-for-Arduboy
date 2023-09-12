#pragma once
#include <ArdVoice.h>

extern "C"
{
    void ardvoice_play_voice(const char *audio)
    {
        ardvoice.playVoice(audio);
    }
    void ardvoice_play_voice_complex(const char *audio, uint16_t startTime, uint16_t endTime, float speed)
    {
        ardvoice.playVoice(audio, startTime, endTime, speed);
    }
    void ardvoice_stop_voice()
    {
        ardvoice.stopVoice();
    }
    bool ardvoice_is_voice_playing()
    {
        return ardvoice.isVoicePlaying();
    }
}
