#![no_std]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

//Include the Arduboy Library
//Initialize the arduboy object
use arduboy_rust::prelude::*;
use fx_consts::*;
const arduboy: Arduboy2 = Arduboy2::new();

// FX Data
const FX_DATA_PAGE: u16 = 0xff65;
const FX_DATA_BYTES: u32 = 39470;
const mapGfx: u32 = 0x000000;
const mapGfxWidth: u16 = 816;
const mapGfxHeight: u16 = 368;
const whaleGfx: u32 = 0x0092A4;
const whaleGfxWidth: u16 = 107;
const whaleGfxHeight: u16 = 69;

static mut showposition: bool = true;
static mut select: usize = 0;
static mut color: u8 = 0;
static mut x: [i16; 2] = [0, 0];
static mut y: [i16; 2] = [0, 0];
static mut mode: u8 = 0;

//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(120);
    fx::begin_data(FX_DATA_PAGE);
}
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    if arduboy.pressed(A) && arduboy.pressed(B) {
        if arduboy.every_x_frames(120) {
            mode += 1;
            if mode == 5 {
                mode = 0
            }
        }
    }
    if arduboy.just_pressed(B) {
        showposition = !showposition;
    }
    if arduboy.pressed(B) {
        select = 0;
    } else {
        select = 1
    }
    if arduboy.just_pressed(A) {
        color ^= dbmReverse;
    }
    if arduboy.pressed(A) {
        if arduboy.just_pressed(UP) {
            y[select] -= 1;
        }
        if arduboy.just_pressed(DOWN) {
            y[select] += 1;
        }
        if arduboy.just_pressed(LEFT) {
            x[select] -= 1;
        }
        if arduboy.just_pressed(RIGHT) {
            x[select] += 1;
        }
    } else {
        if arduboy.pressed(UP) {
            y[select] -= 1;
        }
        if arduboy.pressed(DOWN) {
            y[select] += 1;
        }
        if arduboy.pressed(LEFT) {
            x[select] -= 1;
        }
        if arduboy.pressed(RIGHT) {
            x[select] += 1;
        }
    }
    fx::draw_bitmap(x[0], y[0], mapGfx, 0, dbmNormal);
    match mode {
        0 => fx::draw_bitmap(x[1], y[1], whaleGfx, 0, dbmMasked | color),
        1 => fx::draw_bitmap(x[1], y[1], whaleGfx, 0, dbfMasked | dbmBlack),
        2 => fx::draw_bitmap(x[1], y[1], whaleGfx, 0, dbfMasked | dbmWhite),
        3 => fx::draw_bitmap(x[1], y[1], whaleGfx, 0, dbfMasked | dbmInvert),
        4 => fx::draw_bitmap(x[1], y[1], whaleGfx, 0, dbfMasked | dbmReverse),
        _ => (),
    }
    if showposition {
        arduboy.set_cursor(0, 0);
        arduboy.print(x[select]);
        arduboy.set_cursor(0, 8);
        arduboy.print(y[select]);
    }
    fx::display_clear();
}
