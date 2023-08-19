#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();

// Progmem data
// 8x16

progmem!(
    pub static pills: [u8; _] = [
        // width, height,
        8, 8, 0x7e, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x7e, // TILE 00
        0x7e, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7e, // TILE 01
        0x7e, 0xd5, 0xab, 0xd5, 0xab, 0xd5, 0xab, 0x7e, // TILE 02
    ];
    pub static enemies: [u8; _] = [
        8, 8, // width, height,
        0xa3, 0x51, 0xa6, 0x51, 0xa1, 0x56, 0xa1, 0x53, // TILE 00
        0x7e, 0xdf, 0xb3, 0xdf, 0xbf, 0xd3, 0xbf, 0x7e, // TILE 01
        0x7e, 0xc1, 0xad, 0xc1, 0xa1, 0xcd, 0xa1, 0x7e, // TILE 02
    ];
);
// dynamic ram variables
#[derive(Debug)]
struct Player {
    bitmap: *const u8,
    bitmap_frame: u8,
    x: i16,
    y: i16,
}
#[link_section = ".progmem.data"]
static mut p: Player = Player {
    bitmap: get_sprite_addr!(enemies),
    bitmap_frame: 0,
    x: 0,
    y: 0,
};
progmem!(
    static mut walls: Vec<Player, 100> = Vec::new();
);

unsafe impl Sync for Player {}

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(30);
}

// The loop() function repeats forever after setup() is done
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.clear();
    sprites::draw_override(p.x, p.y, p.bitmap, p.bitmap_frame);
    walls.iter().for_each(|f| {
        sprites::draw_override(f.x, f.y, f.bitmap, f.bitmap_frame);
    });
    arduboy.poll_buttons();
    if arduboy.pressed(UP) {
        p.y -= 1;
    }
    if arduboy.pressed(DOWN) {
        p.y += 1;
    }
    if arduboy.pressed(LEFT) {
        p.x -= 1;
    }
    if arduboy.pressed(RIGHT) {
        p.x += 1;
    }
    if arduboy.just_pressed(A) {
        p.bitmap_frame += 1;
        if p.bitmap_frame > 2 {
            p.bitmap_frame = 0
        }
    }
    if arduboy.just_pressed(B) {
        walls
            .push(Player {
                bitmap: get_sprite_addr!(pills),
                bitmap_frame: 2,
                x: random_between(10, 64) as i16,
                y: random_between(10, 64) as i16,
            })
            .unwrap();
    }

    arduboy.display();
}
