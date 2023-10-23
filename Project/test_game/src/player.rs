use crate::character::{AnimationStateMachine, Character, Sprite, State};
use arduboy_rust::prelude::fx_consts::{
    dbfFlip, dbfMasked, dbmFlip, dbmInvert, dbmMasked, dbmNormal, dbmReverse,
};
use arduboy_rust::prelude::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Player {
    posx: i16,
    posy: i16,
    left: bool,
    sprite: Sprite,
    sprite_left: Sprite,
    animation_state_machine: AnimationStateMachine,
}
impl Character for Player {
    fn game_loop(&mut self, arduboy: Arduboy2) {
        let (mut min, mut max) = self.animation_state_machine.animation_state.get_frames();
        if self.animation_state_machine.animation_state == State::Idle {
            if self.animation_state_machine.idle_state {
                max -= 2
            } else {
                min += 2
            }
        }
        let speed = match self.animation_state_machine.animation_state {
            State::Run => 10,
            State::Idle => 30,
            State::Hurt => 20,
            State::Death => 20,
        };
        if arduboy.every_x_frames(speed) {
            self.sprite.frame += 1;
            if self.sprite.frame < min {
                self.sprite.frame = min;
            }
            if self.sprite.frame > max {
                self.sprite.frame = min;
            }
        }
        self.controller();
        self.animation_state_machine.set_animation_state(arduboy);
    }
    fn draw(&self) {
        if self.left {
            fx::draw_bitmap(
                self.posx,
                self.posy,
                self.sprite_left.Sprite,
                self.sprite.frame,
                self.sprite.mode,
            )
        } else {
            fx::draw_bitmap(
                self.posx,
                self.posy,
                self.sprite.Sprite,
                self.sprite.frame,
                self.sprite.mode,
            )
        }
    }
}
impl Player {
    pub const fn new(Sprite: u32, Sprite_Left: u32, Width: u16, Height: u16) -> Self {
        Player {
            posx: 0,
            posy: 0,
            left: false,
            sprite: Sprite::new(Sprite, Width, Height),
            sprite_left: Sprite::new(Sprite_Left, Width, Height),
            animation_state_machine: AnimationStateMachine::new(),
        }
    }

    pub fn controller(&mut self) {
        if A.just_pressed() {
            self.animation_state_machine.animation_state.next_state()
        }
        if B.just_pressed() {
            self.animation_state_machine.animation_state.last_state()
        }
        if UP.pressed() {
            self.posy -= 1;
        }
        if DOWN.pressed() {
            self.posy += 1;
        }
        if LEFT.pressed() {
            self.posx -= 1;
            self.left = true;
        }
        if RIGHT.pressed() {
            self.posx += 1;
            self.left = false;
        }
    }
}
