#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;

const arduboy: Arduboy2 = Arduboy2::new();
//The setup() function runs once when you turn your Arduboy on
const FX_DATA_PAGE: u16 = 0xfff0;
const FX_DATA_BYTES: u32 = 0;
const FX_SAVE_PAGE: u16 = 0xfff0;
const FX_SAVE_BYTES: u32 = 2;

struct GameState {
    name: &'static str,
    count: u16,
}

static mut gamestate: GameState = GameState {
    name: "\0",
    count: 0,
};

#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    fx::begin_data_save(FX_DATA_PAGE, FX_SAVE_PAGE);
    if fx::load_game_state(&mut gamestate) > 1 {
        gamestate.name = "Hello World !!!\0";
        gamestate.count = 1;
    } else {
        gamestate.name = "Save state Loaded\0";
        gamestate.count += 1;
    }
    fx::save_game_state(&gamestate);
    arduboy.clear();
    arduboy.set_cursor(0, 32 - 8);
    arduboy.print(gamestate.name);
    arduboy.set_cursor(0, 32 + 8);
    arduboy.print("Number of times this\nscetch is run: \0");
    arduboy.print(gamestate.count);
    fx::display();
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    if arduboy.buttons_state() > 0 {
        arduboy.exit_to_bootloader()
    }
}
