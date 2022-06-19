use current::graphics::Frame;
use current::input::InputState;
use current::sprite::{Sprite, Transform};
use current::*;
use glam::Vec2;
use wgpu::Color;

fn main() {
    Rect::run(GameInit::default());
}

struct Rect {
    rect: Sprite,
    i: f32,
}

impl Game for Rect {
    fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::RED).with_transform(Transform {
                scale: Vec2::new(64.0, 64.0),
                ..Default::default()
            }),
            i: 64.0,
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(17, InputState::Pressed) {
            self.i += 32.0;
            let mut transform = self.rect.transform;
            transform = transform.with_scale(Vec2::new(self.i, self.i));
            self.rect.set_transform(transform);
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
    }
}
