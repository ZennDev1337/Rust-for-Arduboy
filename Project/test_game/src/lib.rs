#![no_std]
#![allow(non_upper_case_globals)]

mod character;

mod fxdata;
mod player;

//Include the Arduboy Library

use arduboy_rust::prelude::*;
use character::Character;
use fxdata::*;
use player::Player;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data

// dynamic ram variables

static mut player: Player = Player::new(FX_PLAYER, FX_PLAYER_LEFT, 8, 8);

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(60);
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
    game_loop_(&mut player);

    fx::display_clear()
}

unsafe fn game_loop_(character: &mut impl Character) {
    character.game_loop(arduboy);
    character.draw();
}
