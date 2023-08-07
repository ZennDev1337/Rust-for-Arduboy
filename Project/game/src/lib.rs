#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

// Progmem data
//#[link_section = ".progmem.data"]

// dynamic ram variables

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
}
