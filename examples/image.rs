use current::graphics::Frame;
use current::sprite::{Filter, Sprite, Transform};
use current::*;

fn main() {
    Image::run(GameInit::default());
}

struct Image {
    rect: Sprite,
    test: Sprite,
}

impl Game for Image {
    fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_texture_rect(data.graphics, 0).with_transform(Transform {
                translation: (-160.0, 0.0, 0.0).into(),
                scale: (320.0, 480.0).into(),
                ..Default::default()
            }),
            test: Sprite::new_path_rect(data.graphics, "examples/test.png", Filter::Linear)
                .with_transform(Transform {
                    translation: (160.0, 0.0, 0.0).into(),
                    scale: (320.0, 480.0).into(),
                    ..Default::default()
                }),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
        self.test.render_to(&mut frame);
    }
}
