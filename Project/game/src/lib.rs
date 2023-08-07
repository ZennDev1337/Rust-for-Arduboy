#![no_std]
#![allow(non_upper_case_globals)]
#![feature(const_trait_impl)]
use core::cell::LazyCell;

//Include the Arduboy Library
//Initialize the arduboy object
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[link_section = ".progmem.data"]
static lol2: [u8; 29] = [
    0x54, 0x68, 0x61, 0x6E, 0x6B, 0x73, 0x20, 0x61, 0x20, 0x6C, 0x6F, 0x74, 0x0A, 0x79, 0x6F, 0x75,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x20, 0x0A, 0x48, 0x65, 0x72, 0x6F, 0x00,
];
#[link_section = ".progmem.data"]
static lol1: [u8; 10] = LazyCell::new(|| "hallowelt\0".as_bytes().try_into().unwrap());

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.clear();
    arduboy.print(get_string_addr!(lol1));
    arduboy.display();
}
// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
}
