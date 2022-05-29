use current::*;
use current::graphics::Frame;
use current::sprite::Sprite;

fn main() {
    Rect::run();
}

struct Rect {
    rect: Sprite,
}

impl Game for Rect {
	fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics),
        }
	}
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
