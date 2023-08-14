#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

// Progmem data

// dynamic ram variables
static e: EEPROM = EEPROM::new(10);
struct Scorebord {
    player1: u16,
    text: &'static str,
}
static mut s: Scorebord = Scorebord {
    player1: 0,
    text: "lol\0",
};

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(1);
    arduboy.clear();
    e.init(&mut s);
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    if arduboy.just_pressed(B) {
        s.player1 += 1;
        e.put(&s);
    }
    if arduboy.just_pressed(DOWN) {
        s.player1 -= 1;
        e.put(&s);
    }
    if arduboy.just_pressed(A) {
        s.player1 += 1;
        e.get(&mut s);
    }
    arduboy.clear();
    if s.player1 == 5 {
        arduboy.print(f!(b"lolxd\0"));
        s.text = "it works!!!\0";
        e.put(&s)
    } else {
        arduboy.print(f!(b"nope\0"));
        s.text = "lol\0";
        e.put(&s)
    }

    //e.get(&mut s);
    arduboy.print("\n\0");
    arduboy.print("eeprom save: \0");
    let ss: Scorebord = e.get_direct();
    arduboy.print(ss.player1);
    arduboy.print("\nscore save: \0");
    arduboy.print(s.player1);
    arduboy.print("\n \0");
    arduboy.print(s.text);

    arduboy.display();
}
