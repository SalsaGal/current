use std::f32::consts::TAU;

use current::sprite::{Sprite, Transform};
use current::{Game, GameData, GameExt, GameInit};
use wgpu::Color;

fn main() {
    Rotate::run(GameInit::default());
}

struct Rotate {
    sprite: Sprite,
    angle: f32,
}

impl Game for Rotate {
    fn init(data: &mut GameData) -> Self {
        Self {
            sprite: Sprite::new_color_rect(data.graphics, Color::RED).with_transform(
                Transform::scale(data.graphics.pixel_to_screen_size((32.0, 32.0).into())),
            ),
            angle: 0.0,
        }
    }

    fn update(&mut self, data: &mut GameData) {
        self.angle += data.delta_time.as_secs_f32() * TAU / 4.0;

        self.sprite.set_transform(
            Transform::scale(data.graphics.pixel_to_screen_size((32.0, 32.0).into()))
                .with_straight_rotation(self.angle),
        );
    }

    fn render<'a>(&'a mut self, mut frame: current::graphics::Frame<'a>) {
        self.sprite.render_to(&mut frame);
    }
}
