#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
use arduboy_tone::arduboy_tone_pitch::*;
#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();
const sound: ArduboyTones = ArduboyTones::new();

// Progmem data

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(30);
    arduboy.clear();
    sound.volume_mode(VOLUME_ALWAYS_NORMAL);
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    sound.tone3(NOTE_C3, 500, NOTE_D3, 250, NOTE_E3, 1000);
    delay(2000);
    sound.tone2(NOTE_C3, 500, NOTE_D3, 250);
    delay(2000);
    arduboy.display();
}
