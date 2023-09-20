#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;

#[allow(dead_code)]
const arduboy: Arduboy2 = Arduboy2::new();
const WORLD_WIDTH: usize = 14;
const WORLD_HEIGHT: usize = 7;
const PLAYER_SIZE: i16 = 16;
const PLAYER_X_OFFSET: i16 = WIDTH as i16 / 2 - PLAYER_SIZE / 2;
const PLAYER_Y_OFFSET: i16 = HEIGHT as i16 / 2 - PLAYER_SIZE / 2;
const TILE_SIZE: u8 = 16;
const GRASS: u8 = 0;
const WATER: u8 = 1;
const TREES: u8 = 2;
const STONE: u8 = 3;
const world: [[u8; WORLD_WIDTH]; WORLD_HEIGHT] = [
    [
        TREES, GRASS, GRASS, WATER, GRASS, GRASS, GRASS, TREES, GRASS, GRASS, GRASS, GRASS, GRASS,
        TREES,
    ],
    [
        GRASS, WATER, WATER, WATER, GRASS, WATER, GRASS, GRASS, GRASS, GRASS, GRASS, STONE, GRASS,
        GRASS,
    ],
    [
        GRASS, GRASS, GRASS, GRASS, GRASS, WATER, STONE, GRASS, GRASS, GRASS, TREES, GRASS, GRASS,
        GRASS,
    ],
    [
        STONE, GRASS, GRASS, STONE, TREES, WATER, WATER, WATER, GRASS, WATER, WATER, GRASS, TREES,
        GRASS,
    ],
    [
        GRASS, GRASS, GRASS, GRASS, TREES, GRASS, GRASS, GRASS, TREES, WATER, GRASS, GRASS, STONE,
        TREES,
    ],
    [
        GRASS, GRASS, GRASS, WATER, STONE, GRASS, GRASS, TREES, TREES, TREES, GRASS, GRASS, WATER,
        WATER,
    ],
    [
        GRASS, WATER, WATER, TREES, GRASS, WATER, WATER, TREES, TREES, GRASS, GRASS, GRASS, GRASS,
        STONE,
    ],
];
#[derive(Copy, Clone)]
enum Gamestate {
    GameTitle,
    GamePlay,
    GameOver,
    GameHigh,
}
impl Gamestate {
    fn update(&mut self, new_state: Gamestate) {
        *self = new_state;
    }
}

// Progmem data
progmem!(
    static tiles: [u8; _] = [
        16, 16, // width, height,
        //Grass
        0xff, 0x7f, 0xfb, 0xff, 0xff, 0xbf, 0xff, 0xff, 0xf7, 0xff, 0xfd, 0xff, 0xff, 0xf7, 0x7f,
        0xff, 0xdf, 0xff, 0xff, 0xfb, 0x7f, 0xff, 0xff, 0xff, 0xef, 0xfe, 0xff, 0xff, 0xfb, 0xff,
        0x7f, 0xff, //Water
        0x08, 0x10, 0x10, 0x08, 0x10, 0x08, 0x10, 0x10, 0x10, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x20, 0x40, 0x40, 0x20, 0x00, 0x01, 0x02, 0x02, 0x01, 0x02, 0x02, 0x01, 0x02, 0x21,
        0x40, 0x40, //Tree
        0xff, 0x1f, 0x5b, 0x3f, 0xeb, 0xdd, 0xff, 0xf7, 0xbb, 0xef, 0xfd, 0x7f, 0xe3, 0xcb, 0xe3,
        0xff, 0xff, 0xc7, 0x96, 0xc7, 0xff, 0xff, 0xef, 0xfd, 0xff, 0xe3, 0xcb, 0xe3, 0xff, 0xff,
        0x7b, 0xff, //Stone
        0xff, 0xdf, 0x7b, 0x3f, 0x9f, 0x6f, 0x77, 0xab, 0xdb, 0xd7, 0xcd, 0x5f, 0xbf, 0x77, 0xff,
        0xff, 0xff, 0xc1, 0xdc, 0xd3, 0xaf, 0x9f, 0xae, 0xb0, 0xbb, 0xbd, 0xbd, 0xba, 0xd7, 0xcc,
        0x63, 0xff,
    ];
);

// dynamic ram variables
static mut gamestate: Gamestate = Gamestate::GameTitle;
static mut mapx: i16 = 0;
static mut mapy: i16 = 0;
// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(45);
    arduboy.display();
    arduboy.init_random_seed();
    arduboy.clear();
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
    arduboy.clear();
    gameloop();
    arduboy.display();
}
unsafe fn draw_player() {
    arduboy.fill_rect(
        PLAYER_X_OFFSET,
        PLAYER_Y_OFFSET,
        PLAYER_SIZE as u8,
        PLAYER_SIZE as u8,
        Color::Black,
    )
}

unsafe fn draw_world() {
    let tileswide: u8 = WIDTH / TILE_SIZE + 1;
    let tilestall: u8 = HEIGHT / TILE_SIZE + 1;
    for y in 0..tilestall as i16 {
        for x in 0..tileswide as i16 {
            let tilesx: i16 = x - mapx / TILE_SIZE as i16;
            let tilesy: i16 = y - mapy / TILE_SIZE as i16;
            if tilesx >= 0
                && tilesy >= 0
                && tilesx < WORLD_WIDTH as i16
                && tilesy < WORLD_HEIGHT as i16
            {
                sprites::draw_override(
                    x * TILE_SIZE as i16 + mapx % TILE_SIZE as i16,
                    y * TILE_SIZE as i16 + mapy % TILE_SIZE as i16,
                    get_sprite_addr!(tiles),
                    world[tilesy as usize][tilesx as usize],
                )
            }
        }
    }
    arduboy.fill_rect(0, 0, 48, 8, Color::Black);
    arduboy.set_cursor(0, 0);
    arduboy.print(0 - mapx / TILE_SIZE as i16);
    arduboy.print(f!(b",\0"));
    arduboy.print(0 - mapy / TILE_SIZE as i16)
}
fn titlescreen() {
    arduboy.set_cursor(0, 0);
    arduboy.print(f!(b"Title Screen\n\0"));
    if arduboy.just_pressed(A_BUTTON) {
        unsafe { gamestate.update(Gamestate::GamePlay) }
    }
}
unsafe fn player_input() {
    if arduboy.pressed(UP) {
        if mapy < PLAYER_Y_OFFSET {
            mapy += 1
        }
    }
    if arduboy.pressed(DOWN) {
        if PLAYER_Y_OFFSET + PLAYER_SIZE < mapy + TILE_SIZE as i16 * WORLD_HEIGHT as i16 {
            mapy -= 1
        }
    }
    if arduboy.pressed(LEFT) {
        if mapx < PLAYER_X_OFFSET {
            mapx += 1
        }
    }
    if arduboy.pressed(RIGHT) {
        if PLAYER_X_OFFSET + PLAYER_SIZE < mapx + TILE_SIZE as i16 * WORLD_WIDTH as i16 {
            mapx -= 1
        }
    }
}
unsafe fn gameplay() {
    player_input();
    draw_world();
    draw_player();
    if arduboy.just_pressed(A_BUTTON) {
        unsafe { gamestate.update(Gamestate::GameOver) }
    }
}
fn gameover_screen() {
    arduboy.set_cursor(0, 0);
    arduboy.print(f!(b"Game Over Screen\n\0"));
    if arduboy.just_pressed(A_BUTTON) {
        unsafe { gamestate.update(Gamestate::GameHigh) }
    }
}
fn highscore_screen() {
    arduboy.set_cursor(0, 0);
    arduboy.print(f!(b"High Score Screen\n\0"));
    if arduboy.just_pressed(A_BUTTON) {
        unsafe { gamestate.update(Gamestate::GameTitle) }
    }
}

unsafe fn gameloop() {
    match unsafe { gamestate } {
        Gamestate::GameTitle => titlescreen(),
        Gamestate::GamePlay => gameplay(),
        Gamestate::GameOver => gameover_screen(),
        Gamestate::GameHigh => highscore_screen(),
    }
}
