#![no_std]
#![allow(non_upper_case_globals)]
#![feature(const_trait_impl)]

//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::arduboy_tone_pitch::*;
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

// Progmem data
progmem!(
    static text1: [u8; _] = *b"I'm a PROGMEM Text\0";
    static player_sprite1: [u8; _] = [
        16, 16, 0xfe, 0x01, 0x3d, 0x25, 0x25, 0x3d, 0x01, 0x01, 0xc1, 0x01, 0x3d, 0x25, 0x25, 0x3d,
        0x01, 0xfe, 0x7f, 0x80, 0x9c, 0xbc, 0xb0, 0xb0, 0xb2, 0xb2, 0xb3, 0xb0, 0xb0, 0xb0, 0xbc,
        0x9c, 0x80, 0x7f,
    ];
    static tones: [u16; _] = [
        NOTE_E4,
        400,
        NOTE_D4,
        200,
        NOTE_C4,
        400,
        NOTE_REST,
        200,
        NOTE_D4,
        200,
        NOTE_C4,
        300,
        NOTE_REST,
        100,
        NOTE_C4,
        300,
        NOTE_REST,
        100,
        NOTE_E4,
        300,
        NOTE_REST,
        100,
        NOTE_G4,
        300,
        NOTE_REST,
        100,
        NOTE_F4,
        300,
        NOTE_REST,
        100,
        NOTE_A4,
        300,
        NOTE_REST,
        100,
        NOTE_D5H,
        200,
        NOTE_REST,
        200,
        NOTE_D5H,
        200,
        NOTE_REST,
        1500,
        TONES_REPEAT,
    ];
);
//#[link_section = ".progmem.data"]
//static player_sprite1: [u8; 34] = [
//    16, 16, 0xfe, 0x01, 0x3d, 0x25, 0x25, 0x3d, 0x01, 0x01, 0xc1, 0x01, 0x3d, 0x25, 0x25, 0x3d,
//    0x01, 0xfe, 0x7f, 0x80, 0x9c, 0xbc, 0xb0, 0xb0, 0xb2, 0xb2, 0xb3, 0xb0, 0xb0, 0xb0, 0xbc, 0x9c,
//    0x80, 0x7f,
//];
// #[link_section = ".progmem.data"]
// static tones: [u16; 43] = [
//     NOTE_E4,
//     400,
//     NOTE_D4,
//     200,
//     NOTE_C4,
//     400,
//     NOTE_REST,
//     200,
//     NOTE_D4,
//     200,
//     NOTE_C4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_C4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_E4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_G4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_F4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_A4,
//     300,
//     NOTE_REST,
//     100,
//     NOTE_D5H,
//     200,
//     NOTE_REST,
//     200,
//     NOTE_D5H,
//     200,
//     NOTE_REST,
//     1500,
//     TONES_REPEAT,
// ];

// dynamic ram variables
static mut playerx: c_int = 5;
static mut playery: c_int = 10;

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.clear();
    arduboy.set_frame_rate(60);
    // load sound sequence from progmem
    sound.tones(get_tones_addr!(tones));
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
        if arduboy.audio_enabled() {
            arduboy.audio_off()
        } else {
            arduboy.audio_on()
        }
    }
    arduboy.clear();
    arduboy.set_cursor((WIDTH as i16 / 2) - (text1.len() as i16 * FONT_SIZE as i16 / 2), 10);
    arduboy.print(get_string_addr!(text1));
    sprites::draw_override(playerx, playery, get_sprite_addr!(player_sprite1), 0);
    arduboy.display();
}
