#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
//Initialize the arduboy object

use arduboy_rust::prelude::*;
use fx_consts::{dbmMasked, dbmNormal};
const arduboy: Arduboy2 = Arduboy2::new();

// FX Data
const FX_DATA_PAGE: u16 = 0xfffe;
const FX_DATA_BYTES: u32 = 392;
const FX_DATA_TILES: u32 = 0x000000;
const FX_DATA_TILES_WIDTH: u16 = 16;
const FX_DATA_TILESHEIGHT: u16 = 16;
const FX_DATA_TILES_FRAMES: u8 = 2;
const FX_DATA_TILEMAP: u32 = 0x000044;
const FX_DATA_BALLS: u32 = 0x000144;
const FX_DATA_BALLS_WIDTH: u16 = 16;
const FX_DATA_BALLSHEIGHT: u16 = 16;

const FRAME_RATE: u8 = 60;
const MAX_BALLS: u8 = 55;
const CIRCLE_POINTS: u8 = 84;
const VISABLE_TILES_PER_COLUMN: u8 = 5;
const VISABLE_TILES_PER_ROW: u8 = 9;

const ballWidth: u8 = 16;
const ballHeight: u8 = 16;
const tilemapWidth: u8 = 16;
const tileWidth: u8 = 16;
const tileHeight: u8 = 16;

static mut circle_points: [Point; CIRCLE_POINTS as usize] = [
    Point { x: -15, y: 0 },
    Point { x: -15, y: 1 },
    Point { x: -15, y: 2 },
    Point { x: -15, y: 3 },
    Point { x: -15, y: 4 },
    Point { x: -14, y: 5 },
    Point { x: -14, y: 6 },
    Point { x: -13, y: 7 },
    Point { x: -13, y: 8 },
    Point { x: -12, y: 9 },
    Point { x: -11, y: 10 },
    Point { x: -10, y: 11 },
    Point { x: -9, y: 12 },
    Point { x: -8, y: 13 },
    Point { x: -7, y: 13 },
    Point { x: -6, y: 14 },
    Point { x: -5, y: 14 },
    Point { x: -4, y: 14 },
    Point { x: -3, y: 15 },
    Point { x: -2, y: 15 },
    Point { x: -1, y: 15 },
    Point { x: 0, y: 15 },
    Point { x: 1, y: 15 },
    Point { x: 2, y: 15 },
    Point { x: 3, y: 15 },
    Point { x: 4, y: 14 },
    Point { x: 5, y: 14 },
    Point { x: 6, y: 14 },
    Point { x: 7, y: 13 },
    Point { x: 8, y: 13 },
    Point { x: 9, y: 12 },
    Point { x: 10, y: 11 },
    Point { x: 11, y: 10 },
    Point { x: 12, y: 9 },
    Point { x: 12, y: 8 },
    Point { x: 13, y: 7 },
    Point { x: 13, y: 6 },
    Point { x: 14, y: 5 },
    Point { x: 14, y: 4 },
    Point { x: 14, y: 3 },
    Point { x: 14, y: 2 },
    Point { x: 14, y: 1 },
    Point { x: 15, y: 0 },
    Point { x: 15, y: -1 },
    Point { x: 15, y: -2 },
    Point { x: 15, y: -3 },
    Point { x: 15, y: -4 },
    Point { x: 14, y: -5 },
    Point { x: 14, y: -6 },
    Point { x: 13, y: -7 },
    Point { x: 13, y: -8 },
    Point { x: 12, y: -9 },
    Point { x: 11, y: -10 },
    Point { x: 10, y: -11 },
    Point { x: 9, y: -12 },
    Point { x: 8, y: -13 },
    Point { x: 7, y: -13 },
    Point { x: 6, y: -14 },
    Point { x: 5, y: -14 },
    Point { x: 4, y: -14 },
    Point { x: 3, y: -15 },
    Point { x: 2, y: -15 },
    Point { x: 1, y: -15 },
    Point { x: 0, y: -15 },
    Point { x: -1, y: -15 },
    Point { x: -2, y: -15 },
    Point { x: -3, y: -15 },
    Point { x: -4, y: -14 },
    Point { x: -5, y: -14 },
    Point { x: -6, y: -14 },
    Point { x: -7, y: -13 },
    Point { x: -8, y: -13 },
    Point { x: -9, y: -12 },
    Point { x: -10, y: -11 },
    Point { x: -11, y: -10 },
    Point { x: -12, y: -9 },
    Point { x: -12, y: -8 },
    Point { x: -13, y: -7 },
    Point { x: -13, y: -6 },
    Point { x: -14, y: -5 },
    Point { x: -14, y: -4 },
    Point { x: -14, y: -3 },
    Point { x: -14, y: -2 },
    Point { x: -14, y: -1 },
];

static mut camera: Point = Point { x: 0, y: 0 };
static mut map_location: Point = Point { x: 16, y: 16 };
struct Ball {
    x: i16,
    y: i16,
    xspeed: i16,
    yspeed: i16,
}

static mut ball: [Ball; MAX_BALLS as usize] = [
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
    Ball {
        x: 0,
        y: 0,
        xspeed: 0,
        yspeed: 0,
    },
];
static mut balls_visible: u8 = MAX_BALLS;
static mut pos: u8 = 0;

static mut tilemap_buffer: [u8; VISABLE_TILES_PER_ROW as usize] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
//The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    arduboy.set_frame_rate(FRAME_RATE);
    fx::begin_data(FX_DATA_PAGE);
    ball.iter_mut().for_each(|b| {
        b.x = random_less_than(113) as i16;
        b.y = random_less_than(49) as i16;
        b.xspeed = 1;
        if random_less_than(100) > 49 {
            b.xspeed = -b.xspeed
        }
        b.yspeed = 1;
        if random_less_than(100) > 49 {
            b.yspeed = -b.yspeed
        }
    })
}
#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    // put your main code here, to run repeatedly:
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    if arduboy.just_pressed(A) && balls_visible < MAX_BALLS {
        balls_visible += 1;
    }
    if arduboy.just_pressed(B) && balls_visible > 0 {
        balls_visible -= 1;
    }

    if arduboy.pressed(UP) && map_location.y > 16 {
        map_location.y -= 1;
    }
    if arduboy.pressed(DOWN) && map_location.y < 176 {
        map_location.y += 1;
    }
    if arduboy.pressed(LEFT) && map_location.x > 16 {
        map_location.x -= 1;
    }
    if arduboy.pressed(RIGHT) && map_location.x < 112 {
        map_location.x += 1;
    }

    camera.x = map_location.x + circle_points[pos as usize].x;
    camera.y = map_location.y + circle_points[pos as usize].y;

    // Draw tilemap
    for y in 0..VISABLE_TILES_PER_COLUMN as i16 {
        fx::read_data_array(
            FX_DATA_TILEMAP,
            (y + camera.y / tileHeight as i16) as u8,
            (camera.x / tileWidth as i16) as u8,
            tileWidth,
            tilemap_buffer.as_ptr(),
            VISABLE_TILES_PER_ROW as usize,
        );

        for x in 0..VISABLE_TILES_PER_ROW as i16 {
            fx::draw_bitmap(
                x * tileWidth as i16 - camera.x % tileWidth as i16,
                y * tileHeight as i16 - camera.y % tileHeight as i16,
                FX_DATA_TILES,
                tilemap_buffer[x as usize],
                dbmNormal,
            )
        }
    }
    if !arduboy.not_pressed(UP)
        && !arduboy.not_pressed(DOWN)
        && !arduboy.not_pressed(LEFT)
        && !arduboy.not_pressed(RIGHT)
    {
        pos += 1 % CIRCLE_POINTS;
        if pos > 80 {
            pos = 0
        }
    }
    for i in 0..balls_visible as usize {
        fx::draw_bitmap(ball[i].x, ball[i].y, FX_DATA_BALLS, 0, dbmMasked);
    }
    for i in 0..balls_visible as usize {
        if ball[i].xspeed > 0 {
            ball[i].x += ball[i].xspeed;
            if ball[i].x > WIDTH - ballWidth as i16 {
                ball[i].x = WIDTH - ballWidth as i16;
                ball[i].xspeed = -ball[i].xspeed;
            }
        } else {
            ball[i].x += ball[i].xspeed;
            if ball[i].x < 0 {
                ball[i].x = 0;
                ball[i].xspeed = -ball[i].xspeed;
            }
        }

        if ball[i].yspeed > 0 {
            ball[i].y += ball[i].yspeed;
            if ball[i].y > HEIGHT - ballHeight as i16 {
                ball[i].y = HEIGHT - ballHeight as i16;
                ball[i].yspeed = -ball[i].yspeed;
            }
        } else {
            ball[i].y += ball[i].yspeed;
            if ball[i].y < 0 {
                ball[i].y = 0;
                ball[i].yspeed = -ball[i].yspeed;
            }
        }
    }
    fx::display_clear();
}
