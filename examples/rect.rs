use current::*;
use current::graphics::Frame;
use current::sprite::Sprite;
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
            rect: Sprite::new_color_rect(data.graphics, Color::GREEN),
        }
	}
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
