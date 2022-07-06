use std::f32::consts::TAU;

use current::graphics::{FontID, Frame};
use current::input::InputState;
use current::sprite::{Filter, Sprite, Transform};
use current::{Game, GameData, GameExt};
use glam::{IVec2, Vec2};
use wgpu::Color;

fn main() {
    Crawl::run(current::GameInit {
        window_title: "Crawler",
        window_fullscreen: true,
        ..Default::default()
    });
}

struct Crawl {
    player_pos: IVec2,
    player_direction: Direction,
    player_sprite: Sprite,

    point_pos: IVec2,
    point_sprite: Sprite,

    font: FontID,
    points_text: Sprite,
    points: u32,
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
        let player_pos = IVec2::ZERO;
        let point_pos = IVec2::new(5, 8);
        let font = data
            .graphics
            .load_font("examples/LiberationSans-Regular.ttf");
        Self {
            player_pos,
            player_direction: Direction::Up,
            player_sprite: Sprite::new_path_rect(
                data.graphics,
                "examples/test.png",
                Filter::Linear,
            )
            .with_transform(position_transform(player_pos, Direction::Up)),

            point_pos,
            point_sprite: Sprite::new_color_rect(
                data.graphics,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.5,
                },
            )
            .with_transform(position_transform(point_pos, Direction::Up)),

            font,
            points_text: Sprite::new_text_rect(
                data.graphics,
                font,
                "Points: 0",
                24,
                Color::WHITE,
                Filter::Linear,
            ),
            points: 0,
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
            self.player_sprite
                .set_transform(position_transform(self.player_pos, self.player_direction));
            if self.player_pos == self.point_pos {
                self.points += 1;
                println!("Points: {}", self.points);
                self.points_text = Sprite::new_text_rect(
                    data.graphics,
                    self.font,
                    &format!("Points: {}", self.points),
                    24,
                    Color::WHITE,
                    Filter::Linear,
                )
            }
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.player_sprite.render_to(&mut frame);
        self.point_sprite.render_to(&mut frame);
        self.points_text.render_to(&mut frame);
    }
}

fn position_transform(pos: IVec2, direction: Direction) -> Transform {
    Transform {
        translation: (pos.as_vec2() * Vec2::new(32.0, 32.0)).extend(0.0),
        scale: Vec2::new(32.0, 32.0),
        ..Default::default()
    }
    .with_straight_rotation(direction.into())
}
