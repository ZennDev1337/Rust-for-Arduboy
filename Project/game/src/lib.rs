#![no_std]
#![allow(non_upper_case_globals)]
//Include the Arduboy Library
//Initialize the arduboy object
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

//#[link_section = ".progmem.data"]

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
