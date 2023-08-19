#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();
mod enemies;
mod utils;
// Progmem data
progmem!(
    static pills: [u8; _] = [
        // width, height,
        8, 8, 0x7e, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x7e, // TILE 00
        0x7e, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7e, // TILE 01
        0x7e, 0xd5, 0xab, 0xd5, 0xab, 0xd5, 0xab, 0x7e, // TILE 02
    ];
    static enemies: [u8; _] = [
        8, 8, // width, height,
        0xa3, 0x51, 0xa6, 0x51, 0xa1, 0x56, 0xa1, 0x53, // TILE 00
        0x7e, 0xdf, 0xb3, 0xdf, 0xbf, 0xd3, 0xbf, 0x7e, // TILE 01
        0x7e, 0xc1, 0xad, 0xc1, 0xa1, 0xcd, 0xa1, 0x7e, // TILE 02
    ];
);
// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(20);
    arduboy.clear();
    arduboy.display();
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
}
