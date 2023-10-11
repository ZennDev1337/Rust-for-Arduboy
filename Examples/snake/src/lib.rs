#![no_std]
#![allow(non_upper_case_globals)]
use arduboy_rust::prelude::*;
const arduboy: Arduboy2 = Arduboy2::new();
const sound: ArduboyTones = ArduboyTones::new();
#[allow(dead_code)]
#[repr(C)]
struct Scorebord {
    places: [u16; 3],
}
impl Scorebord {
    fn check_score(&self, score: u16) -> bool {
        self.places[2] < score
    }
    fn update_score(&mut self, score: u16) {
        match score {
            s if self.places[0] < s => {
                self.places[2] = self.places[1];
                self.places[1] = self.places[0];
                self.places[0] = s
            }
            s if self.places[1] < s => {
                self.places[2] = self.places[1];
                self.places[1] = s
            }
            s if self.places[2] < s => self.places[2] = s,
            _ => (),
        }
    }
}

static mut scoreboard: Scorebord = Scorebord { places: [0, 0, 0] };
static eeprom: EEPROM = EEPROM::new(101);
const WORLD_WIDTH: u8 = 32;
const WORLD_HEIGHT: u8 = 16;
const SCALE_FACTOR: u8 = 4;
#[allow(dead_code)]
enum State {
    Title,
    Game,
    Win,
    Scorebord,
    GameOver,
    Reset,
    Pause,
}
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
struct Snake {
    game_state: State,
    points: u16,
    len: u8,
    pos: [(u8, u8); 255 as usize],
    next_food: (u8, u8),
    delta_time: u8,
    direction: Direction,
    last_direction: Direction,
}
impl Snake {
    fn movement(&mut self) {
        for i in 0..(self.len - 1) as usize {
            self.pos[i] = self.pos[i + 1];
        }
        let (x, y) = self.pos[(self.len - 1) as usize];
        match self.direction {
            Direction::Up => {
                self.pos[(self.len - 1) as usize] = (x, y - 1);
            }
            Direction::Left => {
                self.pos[(self.len - 1) as usize] = (x - 1, y);
            }
            Direction::Down => {
                self.pos[(self.len - 1) as usize] = (x, y + 1);
            }
            Direction::Right => {
                self.pos[(self.len - 1) as usize] = (x + 1, y);
            }
        }
        self.last_direction = self.direction;
    }
    unsafe fn get_new_dir(&mut self) {
        if UP.just_pressed() && self.last_direction != Direction::Down {
            self.direction = Direction::Up
        }
        if LEFT.just_pressed() && self.last_direction != Direction::Right {
            self.direction = Direction::Left
        }
        if DOWN.just_pressed() && self.last_direction != Direction::Up {
            self.direction = Direction::Down
        }
        if RIGHT.just_pressed() && self.last_direction != Direction::Left {
            self.direction = Direction::Right
        }
    }
    fn new_food(&mut self) {
        self.next_food.0 = 0;
        self.next_food.1 = 0;
        while self.next_food.0 == 0 && self.next_food.1 == 0 {
            self.next_food.0 = random_between(2, (WORLD_WIDTH - 2).into()) as u8;
            self.next_food.1 = random_between(2, (WORLD_HEIGHT - 2).into()) as u8;
            for i in 0..(self.len) as usize {
                if self.pos[i] == self.next_food {
                    self.next_food.0 = 0;
                    self.next_food.1 = 0;
                }
            }
        }
    }
    unsafe fn render(&self) {
        arduboy.draw_circle(
            ((self.next_food.0 * SCALE_FACTOR) + SCALE_FACTOR / 4)
                .try_into()
                .unwrap(),
            ((self.next_food.1 * SCALE_FACTOR) + SCALE_FACTOR / 4)
                .try_into()
                .unwrap(),
            SCALE_FACTOR / 4,
            Color::White,
        );

        self.pos.iter().enumerate().for_each(|(i, p)| {
            if i == (self.len) as usize {
                return;
            }
            if *p == (0, 0) {
                return;
            }
            arduboy.fill_rect(
                (p.0 as u8 * SCALE_FACTOR).try_into().unwrap(),
                (p.1 as u8 * SCALE_FACTOR).try_into().unwrap(),
                SCALE_FACTOR / 2,
                SCALE_FACTOR / 2,
                Color::White,
            );
        });
    }
    unsafe fn init(&mut self) {
        self.points = 0;
        self.len = 3;
        self.direction = Direction::Right;
        self.last_direction = Direction::Left;
        self.pos = [(0, 0); 255 as usize];
        self.pos[0] = (3, 4);
        self.pos[1] = (4, 4);
        self.pos[2] = (5, 4);
    }
    unsafe fn boarder(&self) {
        for x in 0..WORLD_WIDTH as usize {
            for y in 0..WORLD_HEIGHT as usize {
                if x == 0
                    || y == 0
                    || x == (WORLD_WIDTH - 1) as usize
                    || y == (WORLD_HEIGHT - 1) as usize
                {
                    let scale: u8 = 2;
                    if self.delta_time % 20 == 0 {
                        //scale = 1;
                    };
                    arduboy.fill_rect(
                        (x as u8 * SCALE_FACTOR + scale / 2).try_into().unwrap(),
                        (y as u8 * SCALE_FACTOR + scale / 2).try_into().unwrap(),
                        SCALE_FACTOR / scale,
                        SCALE_FACTOR / scale,
                        Color::White,
                    );
                }
            }
        }
    }
    fn collision_check(&mut self) {
        let (x, y) = self.pos[(self.len - 1) as usize];
        if x == 0 || x == 31 || y == 0 || y == 15 {
            self.game_state = State::GameOver;
        }
        self.pos.iter().enumerate().for_each(|(i, p)| {
            if p == &(x, y) && i != (self.len - 1) as usize {
                self.game_state = State::GameOver;
            }
        });
        if self.pos[(self.len - 1) as usize] == self.next_food {
            self.points += 1;
            self.new_food();
            self.pos[self.len as usize] = self.pos[(self.len - 1) as usize];
            self.len += 1;
            sound.tone(0xff, 0x3f);
        }
    }
}
#[allow(non_upper_case_globals)]
static mut snake: Snake = Snake {
    game_state: State::Reset,
    len: 3,
    points: 0,
    pos: [(0, 0); 255 as usize],
    next_food: (0, 0),
    delta_time: 0,
    direction: Direction::Right,
    last_direction: Direction::Left,
};

#[no_mangle]
pub unsafe extern "C" fn setup() {
    arduboy.begin();
    eeprom.init(&mut scoreboard);
    arduboy.init_random_seed();
    arduboy.set_frame_rate(60);
    arduboy.clear();
}

#[no_mangle]
#[export_name = "loop"]
pub unsafe extern "C" fn loop_() {
    if !arduboy.next_frame() {
        return;
    }
    arduboy.poll_buttons();
    arduboy.clear();
    match snake.game_state {
        State::Reset => {
            snake.init();
            snake.new_food();
            snake.game_state = State::Title;
        }
        State::Title => {
            arduboy.set_cursor(0, 0);
            arduboy.set_text_size(2);
            arduboy.print(f!(b"RustySnake\n\0"));
            arduboy.set_text_size(1);
            arduboy.print(f!(b"\nControls: \nB Pause / A&B reset\n\0"));
            arduboy.print(f!(b"Press B for Scorebord\0"));
            arduboy.print(f!(b"\nZennDev 2023\n\0"));
            if A.just_pressed() {
                snake.game_state = State::Game;
            }
            if B.just_pressed() {
                snake.game_state = State::Scorebord;
            }
        }
        State::Game => {
            snake.get_new_dir();
            if snake.delta_time % 10 == 0 {
                snake.movement();
                snake.collision_check();
            };
            snake.render();
            snake.boarder();
            if B.just_pressed() {
                sound.tone(800, 100);
                snake.game_state = State::Pause;
            }
        }
        State::Win => (),
        State::Pause => {
            let msg = "[ Break ]\0";
            let l = msg.len() as i16 * FONT_WIDTH / 2;
            arduboy.set_cursor(WIDTH / 2 - l, HEIGHT / 2);
            snake.render();
            snake.boarder();
            arduboy.print(msg);
            if B.just_pressed() {
                snake.game_state = State::Game;
                sound.tone(800, 100);
            }
        }
        State::GameOver => {
            arduboy.set_cursor(0, 0);
            if scoreboard.check_score(snake.points) {
                eeprom.put(&scoreboard);
                arduboy.print(f!(b"New Highscore!\0"));
                arduboy.print(f!(b"\nYou are under the\ntop three player\0"));
                arduboy.print(f!(b"\n\nYour Score: \0"));
                arduboy.print(snake.points as u16);
                arduboy.print(f!(b"\n\0"));
                arduboy.print(f!(b"\nPress A to save the \nscore and play again\0"));
            } else {
                arduboy.print(f!(b"Game Over!\0"));
                arduboy.print(f!(b"\n\n\0"));
                arduboy.print(f!(b"Score: \0"));
                arduboy.print(snake.points as u16);
            }
            if A.just_pressed() {
                scoreboard.update_score(snake.points);
                snake.game_state = State::Reset;
            }
        }
        State::Scorebord => {
            arduboy.set_cursor(0, 10);
            arduboy.print(f!(b"1 place: \0"));
            arduboy.print(scoreboard.places[0]);
            arduboy.print(f!(b"\n\n2 place: \0"));
            arduboy.print(scoreboard.places[1]);
            arduboy.print(f!(b"\n\n3 place: \0"));
            arduboy.print(scoreboard.places[2]);
            if A.just_pressed() || B.just_pressed() {
                snake.game_state = State::Title;
            }
        }
    }
    if (A | B).pressed() {
        snake.game_state = State::Reset;
    }
    arduboy.display();
    if snake.delta_time == 60 {
        snake.delta_time = 1
    } else {
        snake.delta_time += 1;
    }
}
