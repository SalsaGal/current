use std::f32::consts::TAU;

use current::sprite::{Sprite, Transform};
use current::{Game, GameData, GameExt};
use glam::Vec2;
use wgpu::Color;

fn main() {
    Rotate::run();
}

struct Rotate {
    sprite: Sprite,
    back: Sprite,
    angle: f32,
}

impl Game for Rotate {
    fn init(data: &mut GameData) -> Self {
        data.set_window_size((512, 512).into());
        Self {
            sprite: Sprite::new_color_rect(data.graphics, Color::RED)
                .with_transform(Transform::scale(Vec2::new(32.0, 32.0))),
            back: Sprite::new_color_rect(data.graphics, Color::GREEN).with_transform(
                Transform::scale(Vec2::new(64.0, 64.0)).with_translation((0.0, 0.0, -1.0).into()),
            ),
            angle: 0.0,
        }
    }

    fn update(&mut self, data: &mut GameData) {
        self.angle += data.delta_time.as_secs_f32() * TAU / 4.0;

        self.sprite.set_transform(
            Transform::scale(Vec2::new(32.0, 32.0)).with_straight_rotation(self.angle),
        );
    }

    fn render<'a>(&'a mut self, mut frame: current::graphics::Frame<'a>) {
        self.sprite.render_to(&mut frame);
        self.back.render_to(&mut frame);
    }
}
