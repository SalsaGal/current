use current::*;
use current::graphics::Frame;
use current::sprite::Sprite;

fn main() {
    Image::run();
}

struct Image {
    rect: Sprite,
}

impl Game for Image {
	fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_texture_rect(data.graphics),
        }
	}
    
    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
