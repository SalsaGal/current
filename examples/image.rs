use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use current::*;

fn main() {
    Image::run();
}

struct Image {
    rect: Sprite,
    test: Sprite,
}

impl Game for Image {
    fn init(data: &mut GameData) -> Self {
        let image = image::load_from_memory(include_bytes!("test.png")).unwrap();
        data.graphics.texture_manager.make_texture(&data.graphics.device, &data.graphics.queue, image);

        Self {
            rect: Sprite::new_texture_rect(data.graphics, 0).with_transform(Transform {
                translation: (-0.5, 0.0, 0.0).into(),
                scale: (0.5, 1.0).into(),
                ..Default::default()
            }),
            test: Sprite::new_texture_rect(data.graphics, 1).with_transform(Transform {
                translation: (0.5, 0.0, 0.0).into(),
                scale: (0.5, 1.0).into(),
                ..Default::default()
            }),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
        self.test.render_to(&mut frame);
    }
}
