use crate::*;

pub unsafe fn gameloop() {
    arduboy.set_cursor(70, 0);
    arduboy.print(get_string_addr!(overlay_score));
    arduboy.print(p.counter as i16);
    match p.live {
        3 => sprites::draw_override(0, 0, get_sprite_addr!(overlay_3hearts), 0),
        2 => sprites::draw_override(0, 0, get_sprite_addr!(overlay_2hearts), 0),
        1 => sprites::draw_override(0, 0, get_sprite_addr!(overlay_1heart), 0),
        _ => (),
    }
    if p.active {
        sprites::draw_override(p.rect.x, p.rect.y, p.bitmap, p.bitmap_frame as u8);
    }

    if p.immortal {
        p.immortal_frame_count += 1;
        if arduboy.every_x_frames(10) {
            p.active = !p.active
        }
        if p.immortal_frame_count == 60 {
            p.immortal_frame_count = 0;
            p.immortal = false;
            p.active = true
        }
    }

    vec_enemies.iter_mut().for_each(|f| {
        if f.active {
            sprites::draw_override(f.rect.x * 8, f.rect.y * 8, f.bitmap, f.bitmap_frame);
        }

        if arduboy.every_x_frames(p.speed as u8) && f.active {
            f.move_down();
            if f.rect.x < 0 {
                f.active = false;
                p.live -= 1;
            }
        }

        let frect = Rect {
            x: f.rect.x * 8,
            y: f.rect.y * 8,
            width: 8,
            height: 8,
        };
        if !p.immortal && f.active {
            if arduboy.collide_rect(p.rect, frect) {
                if p.bitmap_frame as u8 == f.bitmap_frame {
                    f.active = false;
                    p.speed_change = false;
                    p.counter += 1;
                } else {
                    p.live -= 1;
                    p.immortal = true;
                }
            }
        }
    });
    if arduboy.every_x_frames((p.speed * 2) as u8) {
        if enemy_count > 8 {
            let _ = vec_enemies.remove(0);
        }
        vec_enemies
            .push(Enemy {
                active: true,
                bitmap: get_sprite_addr!(enemies),
                bitmap_frame: random_less_than(3) as u8,
                rect: Rect {
                    x: random_between(15, 16) as i16,
                    y: random_between(1, 8) as i16,
                    width: 8,
                    height: 8,
                },
            })
            .unwrap();
        enemy_count += 1
    }
    if p.live == 0 {
        let score = scorebord.check_score(p.counter);
        if score > 0 {
            scorebord.update_score(p.counter)
        }
        p.gamemode = GameMode::Losescreen;
    }
    if arduboy.pressed(UP) {
        if p.rect.y > 7 {
            p.rect.y -= 1;
        }
    }
    if arduboy.pressed(DOWN) {
        if p.rect.y < 56 {
            p.rect.y += 1;
        }
    }
    if arduboy.pressed(LEFT) {
        if p.rect.x > 0 {
            p.rect.x -= 1;
        }
    }
    if arduboy.pressed(RIGHT) {
        if p.rect.x < 120 {
            p.rect.x += 1;
        }
    }
    if arduboy.just_pressed(A) {
        p.bitmap_frame += 1;
        if p.bitmap_frame > 2 {
            p.bitmap_frame = 0
        }
    }
    if arduboy.just_pressed(B) {
        p.bitmap_frame -= 1;
        if p.bitmap_frame < 0 {
            p.bitmap_frame = 2
        }
    }
    if p.counter % 5 == 0 && p.counter != 0 && !p.speed_change {
        p.speed_change = true;
        p.speed -= 1
    }
    // if p.counter == 30 {
    //     p.level += 1;
    //     p.gamemode = GameMode::Winscreen;
    // }
}

progmem!();
