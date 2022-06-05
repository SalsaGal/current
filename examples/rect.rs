use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use current::*;
use glam::Vec2;
use wgpu::Color;

fn main() {
    Rect::run(GameInit::default());
}

struct Rect {
    rect: Sprite,
}

impl Game for Rect {
    fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::GREEN).with_transform(Transform {
                scale: Vec2::new(0.5, 0.5),
                ..Default::default()
            }),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
    }
}
