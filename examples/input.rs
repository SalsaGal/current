use current::*;
use current::graphics::Frame;
use current::input::InputState;
use current::sprite::{Sprite, Transform};
use glam::Vec3;
use wgpu::Color;

fn main() {
    Rect::run();
}

struct Rect {
    rect: Sprite,
    i: f32,
}

impl Game for Rect {
	fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::RED).with_transform(Transform {
                translation: Vec3::new(-0.25, -0.25, 0.0),
                scale: Vec3::new(0.5, 0.5, 1.0),
                ..Default::default()
            }),
            i: 0.5,
        }
	}

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(17, InputState::Pressed) {
            self.i += 0.25;
            self.rect.set_transform(Transform {
                translation: Vec3::new(-self.i / 2.0, -self.i / 2.0, 0.0),
                scale: Vec3::new(self.i, self.i, 1.0),
                ..Default::default()
            });
        }
    }
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
