use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use current::*;

fn main() {
    Image::run();
}

struct Image {
    rect: Sprite,
}

impl Game for Image {
    fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_texture_rect(data.graphics, 0).with_transform(Transform {
                translation: (-0.5, 0.0, 0.0).into(),
                scale: (0.5, 1.0).into(),
                ..Default::default()
            }),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
    }
}
