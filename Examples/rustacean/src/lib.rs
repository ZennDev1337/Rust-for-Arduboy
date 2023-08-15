#![no_std]
#![feature(c_size_t)]
#![allow(non_upper_case_globals)]
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();
const sound: ArduboyTones = ArduboyTones::new();

const BOTTOM: u8 = 64;
const RIGHT_END: u8 = 128;

const CHAR_WIDTH: u8 = 6;
const CHAR_HEIGHT: u8 = 8;

const MSG: &str = "  I'm now a \0";
const MSG2: &str = "Rustacean <3\0";

struct Environment {
    x: u8,
    y: u8,
    msg_len: u8,
}
impl Environment {
    unsafe fn setup(&mut self) {
        arduboy.begin();
        arduboy.set_frame_rate(60);
        let msg_len = MSG.len();
        debug_assert!(msg_len <= (core::u8::MAX as c_size_t));
        self.msg_len = msg_len as u8;
    }

    unsafe fn loop_(&mut self) {
        if !arduboy.next_frame() {
            return;
        }

        if UP.pressed() && self.y > 0 {
            self.y -= 1;
        }
        if RIGHT.pressed() && self.x < RIGHT_END - CHAR_WIDTH * self.msg_len {
            self.x += 1;
        }
        if LEFT.pressed() && self.x > 0 {
            self.x -= 1;
        }
        if DOWN.pressed() && self.y < BOTTOM - CHAR_HEIGHT * 2 {
            self.y += 1;
        }

        if (A | B).pressed() {
            sound.tone(0xff, 0x3f);
        }

        arduboy.clear();
        arduboy.set_cursor(self.x.into(), self.y.into());
        arduboy.print(MSG);
        arduboy.set_cursor(self.x.into(), (self.y + 9).into());
        arduboy.print(MSG2);
        arduboy.display();
    }
}

static mut E: Environment = Environment {
    x: 0,
    y: 0,
    msg_len: 0,
};

#[no_mangle]
pub unsafe extern "C" fn setup() {
    E.setup();
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    E.loop_();
}
