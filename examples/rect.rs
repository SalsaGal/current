use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use current::*;
use glam::Vec2;
use wgpu::Color;

fn main() {
    Rect::run();
}

struct Rect {
    rect: Sprite,
}

impl Game for Rect {
    fn init(data: &mut GameData) -> Self {
        data.set_window_size((600, 500).into());
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::GREEN).with_transform(Transform {
                scale: Vec2::new(200.0, 200.0),
                ..Default::default()
            }),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
    }
}
