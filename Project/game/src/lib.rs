#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();
extern "C" {

    #[link_name = "arduino_serial_begin"]
    fn serial_begin(serial: c_ulong);
    // #[link_name = "arduino_serial_println_chars"]
    // fn serial_println_chars(cstr: *const c_char);
}
// Progmem data

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(30);
    arduboy.clear();
    serial_begin(9600)
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    serial::print("velo\0");
    //serial_println_chars(b"hallo\0" as *const [u8] as *const i8);
    arduboy.display();
}
