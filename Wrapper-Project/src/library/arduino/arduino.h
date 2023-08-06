#pragma once
#include <Arduboy2.h>

extern "C"
{
    long arduino_random_between(long min, long max);
    long arduino_random_less_than(long max);
    void arduino_delay(unsigned long ms);
}
