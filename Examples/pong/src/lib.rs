#![no_std]
#![allow(non_upper_case_globals)]

use arduboy_rust::prelude::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum GameState {
    Title,
    Gameplay,
    Win,
    Lose,
}

struct Ball {
    x: i16,
    y: i16,
    size: u8,
    right: bool,
    down: bool,
}

impl Ball {
    unsafe fn draw(&self) {
        arduboy.fill_rect(self.x.into(), self.y.into(), self.size, self.size, Color::White);
    }
}

struct Paddle {
    x: i16,
    y: i16,
    width: u8,
    height: u8,
}

impl Paddle {
    unsafe fn draw(&self) {
        arduboy.fill_rect(self.x.into(), self.y.into(), self.width, self.height, Color::White);
    }
}

static mut G: Globals = Globals {
    game_state: GameState::Title,
    player_score: 0,
    ai_score: 0,
    ball: Ball {
        x: 62,
        y: 1,
        size: 4,
        right: true,
        down: true,
    },
    player: Paddle {
        x: 0,
        y: 50,
        width: 2,
        height: 12,
    },
    ai: Paddle {
        x: (WIDTH - 4) as i16,
        y: 50,
        width: 2,
        height: 12,
    },
};

struct Globals {
    game_state: GameState,
    player_score: u8,
    ai_score: u8,
    ball: Ball,
    player: Paddle,
    ai: Paddle,
}

#[no_mangle]
pub unsafe extern "C" fn setup() {
    arduboy.begin();
    arduboy.init_random_seed();
    arduboy.set_frame_rate(60);
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    if !arduboy.next_frame() {
        return;
    }

    arduboy.poll_buttons();

    arduboy.clear();
    match G.game_state {
        GameState::Title => {
            arduboy.set_cursor(52, 10);
            arduboy.print(f!(b"PONG\0"));
            arduboy.set_cursor(16, 22);
            arduboy.print(f!(b"Press A to start\0"));
            if A.just_pressed() {
                G.game_state = GameState::Gameplay;
            }
        }
        GameState::Gameplay => {
            gameplay();
            if A.just_pressed() {
                reset_game();
            }
        }
        GameState::Win => {
            arduboy.set_cursor(40, 10);
            arduboy.print(f!(b"You Win!\0"));
            if A.just_pressed() {
                reset_game();
            }
        }
        GameState::Lose => {
            arduboy.set_cursor(37, 10);
            arduboy.print(f!(b"Game Over\0"));
            if A.just_pressed() {
                reset_game();
            }
        }
    }

    arduboy.display();
}

unsafe fn reset_game() {
    G.game_state = GameState::Title;
    G.ball.x = (WIDTH / 2) as i16;
    G.player_score = 0;
    G.ai_score = 0;
}

unsafe fn gameplay() {
    //
    // Player movement
    //

    if UP.pressed() && G.player.y > 0 {
        G.player.y -= 1;
    }

    if DOWN.pressed() && G.player.y + (G.player.height as i16) < (HEIGHT - 1) as i16 {
        G.player.y += 1;
    }

    //
    // AI movement
    //

    if G.ball.x > 115 || random_between(0, 20) == 0 {
        if G.ball.y < G.ai.y {
            G.ai.y -= 1;
        } else if G.ball.y + (G.ball.size as i16) > G.ai.y + G.ai.height as i16 {
            G.ai.y += 1;
        }
    }

    //
    // Ball movement
    //

    if G.ball.y == 1 {
        G.ball.down = true;
    } else if G.ball.y + G.ball.size as i16 == (HEIGHT - 1) as i16 {
        G.ball.down = false;
    }

    if G.ball.x == G.player.x + G.player.width as i16
        && G.ball.y + G.ball.size as i16 > G.player.y
        && G.ball.y < G.player.y + G.player.height as i16
    {
        G.ball.right = true;
    }

    if G.ball.x + G.ball.size as i16 == G.ai.x
        && G.ball.y + G.ball.size as i16 > G.ai.y
        && G.ball.y < G.ai.y + G.ai.height as i16
    {
        G.ball.right = false;
    }

    if G.ball.right {
        G.ball.x += 1;
    } else {
        G.ball.x -= 1;
    }

    if G.ball.down {
        G.ball.y += 1;
    } else {
        G.ball.y -= 1;
    }

    //
    //

    // Scoring
    if G.ball.x + G.ball.size as i16 == -10 {
        G.ai_score += 1;
        G.ball.x = (WIDTH / 2) as i16;
        G.ball.right = true;
    } else if G.ball.x == (WIDTH + 10) as i16 {
        G.player_score += 1;
        G.ball.x = (WIDTH / 2) as i16;
        G.ball.right = false;
    }

    if G.player_score == 5 {
        G.game_state = GameState::Win;
    } else if G.ai_score == 5 {
        G.game_state = GameState::Lose;
    }

    //
    // Drawing
    //

    arduboy.set_cursor(20, 2);
    arduboy.print(G.player_score as u16);

    arduboy.set_cursor(101, 2);
    arduboy.print(G.ai_score as u16);

    arduboy.draw_fast_hline(0, 0, WIDTH, Color::White);
    arduboy.draw_fast_hline(0, (HEIGHT - 1) as i16, WIDTH, Color::White);

    G.player.draw();
    G.ai.draw();
    G.ball.draw();
}
