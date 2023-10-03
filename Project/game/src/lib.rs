#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
use arduboy_rust::arduboyfx::*;
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables
const FX_DATA_PAGE: u16 = 0xfffe;
const FX_DATA_BYTES: u32 = 329;
const FXlogo: u32 = 0x000000;
const FXlogoWith: i16 = 115;
const FXlogoHeight: i16 = 16;
const helloWorld: u32 = 0x0000EA;

static mut x: i16 = (WIDTH - FXlogoWith) / 2;
static mut y: i16 = 25;
static mut xDir: i8 = 1;
static mut yDir: i8 = 1;
// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(30);
    arduboyfx_begin_data(FX_DATA_PAGE);
}
// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboyfx_draw_bitmap(x, y, FXlogo, 0, 0);
    x += xDir as i16;
    y += yDir as i16;
    if x == 0 || x == WIDTH - FXlogoWith {
        xDir = -xDir;
    }
    if y == 0 || y == HEIGHT - FXlogoHeight {
        yDir = -yDir;
    }
    arduboyfx_set_cursor(10, 10);
    arduboyfx_draw_string(helloWorld);
    arduboyfx_display_clear();
}
