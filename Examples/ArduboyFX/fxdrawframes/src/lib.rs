#![no_std]
#![allow(non_upper_case_globals)]
//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();

// FX Data
const FX_DATA_PAGE: u16 = 0xfffd;
const FX_DATA_BYTES: u32 = 674;
const ArduboyLogo: u32 = 0x000000;
const ArduboyLogoWidth: u16 = 88;
const ArduboyLogoHeight: u16 = 16;
const FXLogo: u32 = 0x0000B4;
const FXLogoWidth: u16 = 28;
const FXLogoHeight: u16 = 16;
const ArduboyLogo_Frame: u32 = 0x000128;
const ArduboyLogo_LastFrame: u32 = 0x000290;

enum State {
    Init,
    Logo,
    Wait,
    Main,
}
static mut state: State = State::Init;
static mut timer: u8 = 0;
//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    fx::begin_data(FX_DATA_PAGE);
    arduboy.clear();
}
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    match state {
        State::Init => {
            fx::set_frame(ArduboyLogo_Frame, 0);
            state = State::Logo;
        }
        State::Logo => {
            if fx::draw_loaded_frame() == 0 {
                state = State::Wait;
                timer = 60;
            }
        }
        State::Wait => {
            fx::draw_frame(ArduboyLogo_LastFrame);
            timer -= 1;
            if timer == 0 {
                state = State::Main;
            }
        }
        State::Main => {
            if arduboy.just_pressed(A | B) {
                state = State::Init;
            }
        }
    }
    fx::display_clear();
}
