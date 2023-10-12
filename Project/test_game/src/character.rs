use arduboy_rust::prelude::fx_consts::{dbmMasked, dbmNormal};
use arduboy_rust::prelude::*;

pub trait Character {
    fn game_loop(&mut self, arduboy: Arduboy2);
    fn draw(&self);
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sprite {
    pub mode: u8,
    pub frame: u8,
    pub Sprite: u32,
    pub Width: u16,
    pub Height: u16,
}
impl Sprite {
    pub const fn new(Sprite: u32, Width: u16, Height: u16) -> Self {
        Sprite {
            mode: dbmMasked,
            frame: 0,
            Sprite,
            Width,
            Height,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct AnimationStateMachine {
    pub left: bool,
    pub hurt: u8,
    pub animation_state: State,
    pub idle_state: bool,
}
impl AnimationStateMachine {
    pub const fn new() -> Self {
        AnimationStateMachine {
            left: false,
            hurt: 0,
            animation_state: State::Idle,
            idle_state: false,
        }
    }
    pub fn set_animation_state(&mut self, arduboy: Arduboy2) {
        if arduboy.pressed(UP)
            || arduboy.pressed(DOWN)
            || arduboy.pressed(LEFT)
            || arduboy.pressed(RIGHT)
        {
            self.animation_state = State::Run;
        } else if self.hurt > 0 {
            self.hurt -= 1;
            self.animation_state = State::Hurt
        } else {
            self.animation_state = State::Idle;
        }
        if arduboy.every_x_frames(240) {
            self.idle_state = !self.idle_state;
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum State {
    Run,
    Idle,
    Hurt,
    Death,
}
impl State {
    pub fn get_frames(&self) -> (u8, u8) {
        match self {
            State::Run => (0, 3),
            State::Idle => (4, 7),
            State::Hurt => (8, 10),
            State::Death => (11, 14),
        }
    }
    pub fn next_state(&mut self) {
        match self {
            State::Run => *self = State::Idle,
            State::Idle => *self = State::Hurt,
            State::Hurt => *self = State::Death,
            State::Death => *self = State::Run,
        }
    }
    pub fn last_state(&mut self) {
        match self {
            State::Run => *self = State::Death,
            State::Idle => *self = State::Run,

            State::Hurt => *self = State::Idle,
            State::Death => *self = State::Hurt,
        }
    }
}
