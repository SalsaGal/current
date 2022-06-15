use current::graphics::Frame;
use current::sprite::{Sprite, Transform};
use current::*;

fn main() {
    Text::run(GameInit::default());
}

struct Text {
    sprite: Sprite,
}

impl Game for Text {
    fn init(data: &mut GameData) -> Self {
        Self {
            sprite: Sprite::new_label_rect(data.graphics)
                .with_transform(Transform::scale((32.0, 32.0).into())),
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.sprite.render_to(&mut frame);
    }
}
