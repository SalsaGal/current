use current::graphics::FontID;
use current::sprite::{Sprite, Transform};
use current::*;

use wgpu::Color;

fn main() {
    TextDemo::run(GameInit::default());
}

struct TextDemo {
    font: FontID,
    sprite: Sprite,
    text: String,
}

impl Game for TextDemo {
    fn init(data: &mut GameData) -> Self {
        let font = data
            .graphics
            .load_font("examples/LiberationSans-Regular.ttf");
        Self {
            font,
            sprite: Sprite::new_text_rect(data.graphics, font, "Text", 512, Color::GREEN, sprite::Filter::Linear)
                .with_transform(Transform::scale((512.0, 512.0).into())),
            text: "Text".to_owned(),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(31, input::InputState::Pressed) {
            self.text.push('#');
            self.sprite =
                Sprite::new_text_rect(data.graphics, self.font, &self.text, 512, Color::WHITE, sprite::Filter::Linear)
                    .with_transform(Transform::scale((512.0, 512.0).into()));
        }
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        self.sprite.render_to(&mut frame);
    }
}
