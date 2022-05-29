use current::*;
use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use glam::Vec3;
use wgpu::Color;

fn main() {
    Rect::run();
}

struct Rect {
    rect: Sprite,
}

impl Game for Rect {
	fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::GREEN).with_transform(Transform {
                translation: Vec3::new(-0.5, -0.5, 0.0),
                ..Default::default()
            }),
        }
	}
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
