#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(30);
    arduboy.clear();
    serial::begin(9600)
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    if serial::available() > 0 {
        let intcoming_byte = serial::read_as_utf8_str();
        serial::print("I received: \0");

        serial::println(intcoming_byte);
    }
    if arduboy.pressed(A) {
        serial::println("kekw")
    }

    arduboy.display();
}
