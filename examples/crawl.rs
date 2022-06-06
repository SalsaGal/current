use std::f32::consts::TAU;

use current::graphics::Frame;
use current::input::InputState;
use current::sprite::{Sprite, Transform};
use current::{Game, GameData, GameExt};
use glam::{UVec2, Vec3};

fn main() {
    Crawl::run(current::GameInit { window_title: "Crawler" });
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
    Right
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
        Self {
            player_pos: (0, 0).into(),
            player_direction: Direction::Up,
            player_sprite: Sprite::new_path_rect(data.graphics, "examples/test.png")
                .with_transform(player_transform((0, 0).into(), Direction::Up)),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(17, InputState::Pressed) {
            self.player_pos.y += 1;
            self.player_direction = Direction::Up;
            self.player_sprite.set_transform(player_transform(self.player_pos, self.player_direction));
        }
        if data.input.is_key(31, InputState::Pressed) {
            self.player_pos.y -= 1;
            self.player_direction = Direction::Down;
            self.player_sprite.set_transform(player_transform(self.player_pos, self.player_direction));
        }
        if data.input.is_key(32, InputState::Pressed) {
            self.player_pos.x += 1;
            self.player_direction = Direction::Right;
            self.player_sprite.set_transform(player_transform(self.player_pos, self.player_direction));
        }
        if data.input.is_key(30, InputState::Pressed) {
            self.player_pos.x -= 1;
            self.player_direction = Direction::Left;
            self.player_sprite.set_transform(player_transform(self.player_pos, self.player_direction));
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.player_sprite.render_to(&mut frame);
    }
}

fn player_transform(pos: UVec2, direction: Direction) -> Transform {
    let origin = Vec3::new(-1.0 + 0.125, -1.0 + 0.125, 0.0);
    Transform {
        translation: origin + (pos.extend(0).as_vec3() * 0.25),
        scale: (0.125, 0.125).into(),
        ..Default::default()
    }.with_straight_rotation(direction.into())
}
