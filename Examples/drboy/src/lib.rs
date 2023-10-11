#![no_std]
#![allow(non_upper_case_globals)]

//Include the Arduboy Library
#[allow(unused_imports)]
use arduboy_rust::prelude::*;
use tones_pitch::*;
mod gameloop;

#[allow(dead_code)]
pub const arduboy: Arduboy2 = Arduboy2::new();
pub const sound: ArduboyTones = ArduboyTones::new();

pub static eeprom: EEPROM = EEPROM::new(200);
pub static mut scorebord: Scoreboard = Scoreboard {
    player1: 0,
    player2: 0,
    player3: 0,
};
pub struct Scoreboard {
    pub player1: u16,
    pub player2: u16,
    pub player3: u16,
}
impl Scoreboard {
    pub fn check_score(&self, score: u16) -> u8 {
        match score {
            s if s > self.player1 => 1,
            s if s == self.player1 => 2,
            s if s > self.player2 => 2,
            s if s == self.player1 => 3,
            s if s > self.player3 => 3,
            _ => 0,
        }
    }
    pub fn update_score(&mut self, score: u16, e: &EEPROM) {
        let place = self.check_score(score);
        match place {
            1 => {
                self.player3 = self.player2;
                self.player2 = self.player1;
                self.player1 = score;
            }
            2 => {
                self.player3 = self.player2;
                self.player2 = score;
            }
            3 => {
                self.player3 = score;
            }
            _ => (),
        }
        e.put(self)
    }
}
// dynamic ram variables
#[derive(Debug)]
pub struct Enemy {
    pub active: bool,
    pub bitmap: *const u8,
    pub bitmap_frame: u8,
    pub rect: Rect,
}
impl Enemy {
    pub fn move_down(&mut self) {
        self.rect.x -= 1;
    }
}
#[derive(Debug)]
pub struct Player {
    pub gamemode: GameMode,
    pub immortal: bool,
    pub immortal_frame_count: u8,
    pub active: bool,
    pub live: u8,
    pub level: u8,
    pub speed: u8,
    pub speed_change: bool,
    pub counter: u16,
    pub bitmap: *const u8,
    pub bitmap_frame: i8,
    pub rect: Rect,
    pub gameover_height: i16,
    //pub sound: bool,
}

pub static mut p: Player = Player {
    gamemode: GameMode::Titlescreen,
    live: 3,
    level: 1,
    immortal: false,
    immortal_frame_count: 0,
    active: true,
    counter: 0,
    speed: 60,
    speed_change: false,
    bitmap: get_sprite_addr!(player),
    bitmap_frame: 0,
    rect: Rect {
        x: 0,
        y: 8,
        width: 8,
        height: 8,
    },
    gameover_height: -30,
    //sound: true,
};

unsafe impl Sync for Player {}
#[derive(Debug)]
pub enum GameMode {
    Titlescreen,
    GameLoop,
    Losescreen,
    Scoreboard,
    Reset,
}

// The setup() function runs once when you turn your Arduboy on
#[no_mangle]
pub unsafe extern "C" fn setup() {
    // put your setup code here, to run once:
    arduboy.begin();
    eeprom.init(&mut scorebord);
    arduboy.set_frame_rate(60);
    sound.tones(get_tones_addr!(music));
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
    arduboy.poll_buttons();

    match p.gamemode {
        GameMode::Titlescreen => {
            sprites::draw_override(0, 0, get_sprite_addr!(titlescreen), 0);
            if arduboy.just_pressed(A) {
                p.gamemode = GameMode::GameLoop;
            }
            if arduboy.just_pressed(B) {
                p.gamemode = GameMode::Scoreboard;
            }
            if arduboy.just_pressed(DOWN) {
                arduboy.audio_toggle();
            }
        }
        GameMode::GameLoop => {
            gameloop::gameloop();
        }
        GameMode::Losescreen => {
            arduboy.set_text_size(2);
            arduboy.set_cursor(13, p.gameover_height);
            arduboy.print(get_string_addr!(text_gameover));
            if arduboy.every_x_frames(2) && p.gameover_height < 15 {
                p.gameover_height += 1
            }
            if p.gameover_height == 15 {
                arduboy.set_text_size(1);
                arduboy.set_cursor(13, 35);
                arduboy.print(get_string_addr!(text_gameover_score));
                arduboy.print(p.counter as i16);
            }
            if arduboy.just_pressed(A) || arduboy.just_pressed(B) {
                p.gamemode = GameMode::Reset;
            }
        }
        GameMode::Scoreboard => {
            arduboy.set_text_size(2);
            arduboy.set_cursor(0, 0);
            arduboy.print(f!(b"Scoreboard\0"));
            arduboy.set_text_size(1);
            arduboy.print(f!(b"\n\n\n\0"));
            arduboy.print(f!(b"Player 1: \0"));
            arduboy.print(scorebord.player1);
            arduboy.print(f!(b"\n\0"));
            arduboy.print(f!(b"Player 2: \0"));
            arduboy.print(scorebord.player2);
            arduboy.print(f!(b"\n\0"));
            arduboy.print(f!(b"Player 3: \0"));
            arduboy.print(scorebord.player3);
            if arduboy.just_pressed(A) || arduboy.just_pressed(B) {
                p.gamemode = GameMode::Reset
            }
        }
        GameMode::Reset => {
            vec_enemies.iter_mut().for_each(|f| f.active = false);
            p = Player {
                gamemode: GameMode::Titlescreen,
                live: 3,
                level: 1,
                immortal: false,
                immortal_frame_count: 0,
                active: true,
                counter: 0,
                speed: 60,
                speed_change: false,
                bitmap: get_sprite_addr!(player),
                bitmap_frame: 0,
                rect: Rect {
                    x: 0,
                    y: 8,
                    width: 8,
                    height: 8,
                },
                gameover_height: -30,
                //sound: p.sound,
            };
        }
    }

    arduboy.display();
}
pub static mut enemy_count: u8 = 0;
pub static mut vec_enemies: Vec<Enemy, 9> = Vec::<Enemy, 9>::new();
progmem!(
    static text_gameover: [u8; _] = *b"Game Over\0";
    static text_levelwin: [u8; _] = *b"Congrats\0";
    static text_gameover_score: [u8; _] = *b"Your score is: \0";
    pub static player: [u8; _] = [
        8, 8, // width, height,
        0xa3, 0x51, 0xa6, 0x51, 0xa1, 0x56, 0xa1, 0x53, // TILE 00
        0x7e, 0xdf, 0xb3, 0xdf, 0xbf, 0xd3, 0xbf, 0x7e, // TILE 01
        0x7e, 0xc1, 0xad, 0xc1, 0xa1, 0xcd, 0xa1, 0x7e, // TILE 02
    ];
    pub static titlescreen: [u8; _] = [
        // width, height,
        128, 64, 0xff, 0xff, 0xff, 0x3f, 0x1f, 0x0f, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
        0x07, 0x07, 0x07, 0x0f, 0x1f, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x03, 0x03, 0x03, 0x03, 0x03,
        0x07, 0x06, 0x1e, 0xfc, 0xf0, 0x00, 0x00, 0x00, 0xf0, 0xf0, 0x60, 0x30, 0x30, 0x30, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xc3, 0xc3, 0xc3, 0xc3, 0xe7, 0xfe, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x80, 0xe0, 0x60, 0x30,
        0x30, 0x30, 0x30, 0x60, 0xe0, 0x80, 0x00, 0x10, 0xf0, 0xe0, 0x80, 0x00, 0x00, 0x00, 0x80,
        0xe0, 0x70, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x7f, 0x7f, 0x60, 0x60, 0x60, 0x60, 0x60, 0x70, 0x30, 0x3c, 0x1f, 0x07, 0x00, 0x00,
        0x00, 0x7f, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x7f, 0x60, 0x60, 0x60, 0x60, 0x60, 0x71, 0x3f,
        0x1e, 0x00, 0x00, 0x00, 0x0f, 0x3f, 0x30, 0x60, 0x60, 0x60, 0x60, 0x30, 0x3f, 0x0f, 0x00,
        0x00, 0x00, 0x03, 0x0f, 0xfe, 0xf0, 0x3e, 0x0f, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x06, 0x07, 0x03, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x30, 0x10, 0x60, 0x10, 0x10, 0x60, 0x10, 0x30, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x60, 0x56, 0x60, 0x50,
        0x66, 0x50, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x05, 0x0a, 0x05,
        0x0a, 0x05, 0x0a, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x6f, 0x59,
        0x6f, 0x5f, 0x69, 0x5f, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf0, 0xe0, 0xc0, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xc0, 0xe0, 0xf0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xd9, 0xb6, 0xb6, 0xb6, 0xcd, 0xff, 0xc7, 0xbb, 0xbb, 0xbb, 0xff, 0xc7, 0xbb, 0xbb, 0xbb,
        0xc7, 0xff, 0x83, 0xfb, 0xfb, 0xff, 0xff, 0xc7, 0xab, 0xab, 0xab, 0xa7, 0xff, 0xbb, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0xb6, 0xb6, 0xb6, 0xc9, 0xff, 0xff, 0x7f,
        0x3e, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x1c, 0x3e, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0xf6, 0xf6, 0xf6, 0xf9, 0xff,
        0x80, 0xff, 0x9f, 0xab, 0xab, 0xab, 0x87, 0xff, 0x73, 0x4f, 0x3f, 0xcf, 0xf3, 0xff, 0xbb,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xbf, 0xc7, 0xe9, 0xee, 0xe9, 0xc7, 0xbf,
        0xff, 0xff, 0xff, 0xff, 0xff,
    ];

    pub static overlay_score: [u8; _] = *b"Score: \0";

    pub static enemies: [u8; _] = [
        8, 8, // width, height,
        0x7e, 0xd5, 0xab, 0xd5, 0xab, 0xd5, 0xab, 0x7e, // TILE 00 Gray
        0x7e, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7e, // TILE 01 White
        0x7e, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0x7e, // TILE 02 Black
    ];
    pub static overlay_3hearts: [u8; _] = [
        // width, height,
        64, 8, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x00, 0x7a, 0x00, 0x0c, 0x30, 0x40, 0x30, 0x0c,
        0x00, 0x38, 0x54, 0x54, 0x58, 0x00, 0x58, 0x54, 0x54, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x1e, 0x3f, 0x7e, 0xfc, 0x7e, 0x3f, 0x1e, 0x0c, 0x00, 0x00, 0x0c, 0x1e, 0x3f, 0x7e,
        0xfc, 0x7e, 0x3f, 0x1e, 0x0c, 0x00, 0x00, 0x0c, 0x1e, 0x3f, 0x7e, 0xfc, 0x7e, 0x3f, 0x1e,
        0x0c, 0x00, 0x00, 0x00, 0x00,
    ];
    pub static overlay_2hearts: [u8; _] = [
        // width, height,
        64, 8, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x00, 0x7a, 0x00, 0x0c, 0x30, 0x40, 0x30, 0x0c,
        0x00, 0x38, 0x54, 0x54, 0x58, 0x00, 0x58, 0x54, 0x54, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x1e, 0x3f, 0x7e, 0xfc, 0x7e, 0x3f, 0x1e, 0x0c, 0x00, 0x00, 0x0c, 0x1e, 0x3f, 0x7e,
        0xfc, 0x7e, 0x3f, 0x1e, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    pub static overlay_1heart: [u8; _] = [
        // width, height,
        64, 8, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x40, 0x00, 0x7a, 0x00, 0x0c, 0x30, 0x40, 0x30, 0x0c,
        0x00, 0x38, 0x54, 0x54, 0x58, 0x00, 0x58, 0x54, 0x54, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x1e, 0x3f, 0x7e, 0xfc, 0x7e, 0x3f, 0x1e, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    static music: [u16; _] = [
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        236,
        NOTE_REST,
        13,
        NOTE_A2,
        1186,
        NOTE_REST,
        63,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        711,
        NOTE_REST,
        38,
        NOTE_F2,
        236,
        NOTE_REST,
        13,
        NOTE_E2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        1423,
        NOTE_REST,
        576,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_C3,
        473,
        NOTE_REST,
        26,
        NOTE_D3,
        948,
        NOTE_REST,
        51,
        NOTE_C3,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_B2,
        473,
        NOTE_REST,
        26,
        NOTE_G2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        1423,
        NOTE_REST,
        1076,
        NOTE_D3,
        473,
        NOTE_REST,
        26,
        NOTE_D3,
        948,
        NOTE_REST,
        51,
        NOTE_D3,
        473,
        NOTE_REST,
        26,
        NOTE_C3,
        948,
        NOTE_REST,
        51,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_G2,
        473,
        NOTE_REST,
        26,
        NOTE_F2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        1423,
        NOTE_REST,
        1576,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_F2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_G2,
        948,
        NOTE_REST,
        51,
        NOTE_F2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_C2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        1423,
        NOTE_REST,
        1576,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        711,
        NOTE_REST,
        38,
        NOTE_F2,
        236,
        NOTE_REST,
        13,
        NOTE_E2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        1423,
        NOTE_REST,
        576,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_C3,
        473,
        NOTE_REST,
        26,
        NOTE_D3,
        948,
        NOTE_REST,
        51,
        NOTE_C3,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_B2,
        473,
        NOTE_REST,
        26,
        NOTE_G2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        1423,
        NOTE_REST,
        1076,
        NOTE_D3,
        473,
        NOTE_REST,
        26,
        NOTE_D3,
        948,
        NOTE_REST,
        51,
        NOTE_D3,
        473,
        NOTE_REST,
        26,
        NOTE_C3,
        948,
        NOTE_REST,
        51,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_A2,
        498,
        NOTE_REST,
        1,
        NOTE_G2,
        473,
        NOTE_REST,
        26,
        NOTE_F2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        1423,
        NOTE_REST,
        1576,
        NOTE_D2,
        948,
        NOTE_REST,
        51,
        NOTE_A2,
        473,
        NOTE_REST,
        26,
        NOTE_G2,
        948,
        NOTE_REST,
        51,
        NOTE_F2,
        473,
        NOTE_REST,
        26,
        NOTE_E2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        473,
        NOTE_REST,
        26,
        NOTE_C2,
        473,
        NOTE_REST,
        26,
        NOTE_D2,
        1423,
        NOTE_REST,
        1576,
        NOTE_F2,
        1423,
        TONES_REPEAT,
    ];
);
