#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;
use arduboyfx::fx_consts::*;
const arduboy: Arduboy2 = Arduboy2::new();

//FX Data
const FX_DATA_PAGE: u16 = 0xffc9;
const FX_DATA_BYTES: u32 = 13937;
const arduboyFont: u32 = 0x000000;
const arduboyFontWidth: u16 = 6;
const arduboyFontHeight: u16 = 8;
const arduboyFontFrames: u16 = 256;
const maskedFont: u32 = 0x000604;
const maskedFontWidth: u16 = 16;
const maskedFontHeight: u16 = 24;
const maskedFontFrames: u8 = 128;
const helloWorld: u32 = 0x003608;

static mut frames: u16 = 0;
static mut speed: u8 = 1;
static mut scroll_x: i16 = 128;
static mut font_mode: u8 = dcmNormal;
static mut leading_digits: i8 = 5;
static str: &str = "FX Demo\0";
//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    fx::begin_data(FX_DATA_PAGE);
    fx::set_font(arduboyFont, dcmNormal);
    fx::set_cursor_range(0, 32767);
}
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    fx::set_cursor(0, 0);
    fx::set_font_mode(dcmNormal);
    fx::draw_string(str);

    fx::set_cursor(WIDTH - 5 * arduboyFontWidth as i16, 0);
    fx::draw_number(frames, leading_digits);

    fx::set_cursor(scroll_x, 24);
    fx::set_font(maskedFont, dcmMasked | font_mode);
    fx::draw_string(helloWorld);

    fx::set_cursor(13, HEIGHT - arduboyFontHeight as i16);
    fx::set_font(arduboyFont, font_mode);
    fx::draw_string(" Press any button \0");

    fx::display_clear();

    scroll_x -= speed as i16;
    if scroll_x < -1792 {
        scroll_x = 128
    }
    frames += 1;
    if arduboy.just_pressed(ANY_BUTTON) {
        frames = 0
    }
    if arduboy.just_pressed(UP) {
        speed = 2
    }
    if arduboy.just_pressed(DOWN) {
        speed = 1
    }
    if arduboy.just_pressed(LEFT) {
        leading_digits = if leading_digits == -5 { 0 } else { -5 }
    }
    if arduboy.just_pressed(RIGHT) {
        leading_digits = if leading_digits == 5 { 0 } else { 5 }
    }
    if arduboy.just_pressed(A) {
        font_mode = dcmNormal
    }
    if arduboy.just_pressed(B) {
        font_mode = dcmReverse
    }
}
