use current::*;
use current::graphics::Frame;
use current::input::InputState;
use current::sprite::{Sprite, Transform};
use glam::{Vec3, Vec2};
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
                scale: Vec2::new(0.5, 0.5),
                ..Default::default()
            }),
            i: 0.5,
        }
	}

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(17, InputState::Pressed) {
            self.i += 0.25;
            let mut transform = self.rect.transform;
            transform = transform.with_scale(Vec2::new(self.i, self.i)).with_translation_centered(Vec3::ZERO);
            self.rect.set_transform(transform);
        }
    }
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
