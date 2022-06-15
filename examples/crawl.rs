use std::f32::consts::TAU;

use current::graphics::{Frame, Graphics};
use current::input::InputState;
use current::sprite::{Filter, Sprite, Transform};
use current::{Game, GameData, GameExt};
use glam::{UVec2, Vec2};
use wgpu::Color;

fn main() {
    Crawl::run(current::GameInit {
        window_title: "Crawler",
        ..Default::default()
    });
}

struct Crawl {
    player_pos: UVec2,
    player_direction: Direction,
    player_sprite: Sprite,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for f32 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 0.0,
            Direction::Down => TAU / 2.0,
            Direction::Left => TAU / 4.0,
            Direction::Right => 3.0 * TAU / 4.0,
        }
    }
}

impl Game for Crawl {
    fn init(data: &mut GameData) -> Self {
        data.graphics.background_color = Color::BLUE;
        let player_pos = UVec2::new(4, 4);
        Self {
            player_pos,
            player_direction: Direction::Up,
            player_sprite: Sprite::new_path_rect(
                data.graphics,
                "examples/test.png",
                Filter::Linear,
            )
            .with_transform(player_transform(
                data.graphics,
                player_pos,
                Direction::Up,
            )),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        let mut modified = false;
        if data.input.is_key(17, InputState::Pressed) {
            self.player_pos.y += 1;
            self.player_direction = Direction::Up;
            modified = true;
        }
        if data.input.is_key(31, InputState::Pressed) {
            self.player_pos.y -= 1;
            self.player_direction = Direction::Down;
            modified = true;
        }
        if data.input.is_key(32, InputState::Pressed) {
            self.player_pos.x += 1;
            self.player_direction = Direction::Right;
            modified = true;
        }
        if data.input.is_key(30, InputState::Pressed) {
            self.player_pos.x -= 1;
            self.player_direction = Direction::Left;
            modified = true;
        }

        if modified {
            self.player_sprite.set_transform(player_transform(
                data.graphics,
                self.player_pos,
                self.player_direction,
            ));
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.player_sprite.render_to(&mut frame);
    }
}

fn player_transform(graphics: &Graphics, pos: UVec2, direction: Direction) -> Transform {
    Transform {
        translation: graphics
            .pixel_to_screen_pos(pos.as_vec2() * Vec2::new(32.0, 32.0))
            .extend(0.0),
        scale: graphics.pixel_to_screen_size(Vec2::new(32.0, 32.0)),
        ..Default::default()
    }
    .with_straight_rotation(direction.into())
}
