#![no_std]
#![allow(non_upper_case_globals)]
//Include the Arduboy Library
//Initialize the arduboy object
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();

#[link_section = ".progmem.data"]
static background_sprite: [u8; 10] = [8, 8, 0x81, 0x00, 0x12, 0x40, 0x04, 0x11, 0x00, 0x04];
#[link_section = ".progmem.data"]
static player_sprite1: [u8; 34] = [
    16, 16, 0xfe, 0x01, 0x3d, 0x25, 0x25, 0x3d, 0x01, 0x01, 0xc1, 0x01, 0x3d, 0x25, 0x25, 0x3d,
    0x01, 0xfe, 0x7f, 0x80, 0x9c, 0xbc, 0xb0, 0xb0, 0xb2, 0xb2, 0xb3, 0xb0, 0xb0, 0xb0, 0xbc, 0x9c,
    0x80, 0x7f,
];
#[link_section = ".progmem.data"]
static player_sprite2: [u8; 34] = [
    16, 16, 0xfc, 0x02, 0x19, 0x25, 0x25, 0x19, 0x01, 0x01, 0x01, 0x01, 0x19, 0x25, 0x25, 0x19,
    0x02, 0xfc, 0x3f, 0x40, 0x80, 0x98, 0x8c, 0x86, 0x82, 0x82, 0x82, 0x82, 0x86, 0x8c, 0x98, 0x80,
    0x40, 0x3f,
];

// Put your variables here
static mut playerx: c_int = 5;
static mut playery: c_int = 10;
static mut playermode: bool = false;
// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.clear();
    arduboy.set_frame_rate(60);
}
// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    if !arduboy.next_frame() {
        return;
    }
    // put your main code here, to run repeatedly:
    arduboy.clear();
    arduboy.poll_buttons();

    if arduboy.pressed(LEFT) {
        playerx -= 1;
    }
    if arduboy.pressed(RIGHT) {
        playerx += 1;
    }
    if arduboy.pressed(UP) {
        playery -= 1;
    }
    if arduboy.pressed(DOWN) {
        playery += 1;
    }
    if arduboy.just_pressed(A) {
        playermode = !playermode
    }

    for i in (0..WIDTH).step_by(8) {
        for j in (0..HEIGHT).step_by(8) {
            sprites::draw_override(i.into(), j.into(), get_sprite_addr!(background_sprite), 0)
        }
    }

    if playermode {
        sprites::draw_override(playerx, playery, get_sprite_addr!(player_sprite1), 0);
    } else {
        sprites::draw_override(playerx, playery, get_sprite_addr!(player_sprite2), 0);
    }

    arduboy.set_cursor(0, 0);
    arduboy.display();
}
