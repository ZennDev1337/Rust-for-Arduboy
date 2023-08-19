#![no_std]
#![allow(non_upper_case_globals)]
//Include the Arduboy Library
//Initialize the arduboy object
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();

// #[link_section = ".progmem.data"]

// Setup eeprom memory
static mut eeprom: EEPROMBYTE = EEPROMBYTE::new(10);

static mut count: u8 = 0;

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    eeprom.init();
    arduboy.clear();
    arduboy.set_frame_rate(30);
}
// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.clear();
    arduboy.poll_buttons();
    if arduboy.just_pressed(UP) {
        count += 1;
    }
    if arduboy.just_pressed(DOWN) {
        count -= 1;
    }
    if arduboy.just_pressed(A) {
        eeprom.update(count)
    }
    arduboy.set_cursor(0, 0);
    arduboy.print(count as u16);

    arduboy.set_cursor(0, 30);
    arduboy.print(f!(b"Counter:\0"));
    arduboy.print(count as u16);
    arduboy.set_cursor(0, 40);
    arduboy.print(f!(b"eeprom:\0"));

    //arduboy.print(eeprom.read() as u16);

    arduboy.display();
}
