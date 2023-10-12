#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library

use arduboy_rust::prelude::*;

extern crate arduboy_rust;
use fx_consts::*;

const FX_DATA_PAGE: u16 = 0xffff;
const FX_DATA_BYTES: u32 = 244;
const FX_DATA_TILES: u32 = 0x000000;
const FX_DATA_TILES_WIDTH: u16 = 8;
const FX_DATA_TILESHEIGHT: u16 = 8;
const FX_DATA_TILES_FRAMES: u8 = 15;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables
static mut lol: u8 = 0;
enum State {
    Run,
    Idle1,
    Idle2,
    Hurt,
    Death,
}
impl State {
    fn get_frames(&self) -> (u8, u8) {
        match self {
            State::Run => (0, 3),
            State::Idle1 => (4, 5),
            State::Idle2 => (6, 7),
            State::Hurt => (8, 10),
            State::Death => (11, 14),
        }
    }
    fn next_state(&mut self) {
        match self {
            State::Run => *self = State::Idle1,
            State::Idle1 => *self = State::Idle2,
            State::Idle2 => *self = State::Hurt,
            State::Hurt => *self = State::Death,
            State::Death => *self = State::Run,
        }
    }
}
static mut state: State = State::Run;
// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    fx::begin_data(FX_DATA_PAGE);
    arduboy.clear()
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
    if A.just_pressed() {
        state.next_state();
    }
    let (min, max) = state.get_frames();
    if arduboy.every_x_frames(10) {
        lol += 1;
        if lol < min {
            lol = min;
        }
        if lol > max {
            lol = min;
        }
    }
    fx::draw_bitmap(0, 0, FX_DATA_TILES, lol, dbmNormal);
    fx::display_clear()
}
