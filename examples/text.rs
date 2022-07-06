use current::graphics::FontID;
use current::sprite::Sprite;
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
        data.graphics.frame_size = Some((1000.0, 1000.0).into());
        let font = data
            .graphics
            .load_font("examples/LiberationSans-Regular.ttf");
        Self {
            font,
            sprite: Sprite::new_text_rect(
                data.graphics,
                font,
                "Text",
                256,
                Color::GREEN,
                sprite::Filter::Linear,
            ),
            text: "Text".to_owned(),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(31, input::InputState::Pressed) {
            self.text.push('#');
            self.sprite = Sprite::new_text_rect(
                data.graphics,
                self.font,
                &self.text,
                256,
                Color::WHITE,
                sprite::Filter::Linear,
            )
        }
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        self.sprite.render_to(&mut frame);
    }
}
