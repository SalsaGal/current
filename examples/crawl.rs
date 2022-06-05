use current::graphics::Frame;
use current::input::InputState;
use current::sprite::{Sprite, Transform};
use current::{Game, GameData, GameExt};
use glam::{UVec2, Vec3};

fn main() {
    Crawl::run(current::GameInit { window_title: "Crawler" });
}

struct Crawl {
    player_pos: UVec2,
    player_sprite: Sprite,
}

impl Game for Crawl {
    fn init(data: &mut GameData) -> Self {
        Self {
            player_pos: (0, 0).into(),
            player_sprite: Sprite::new_path_rect(data.graphics, "examples/test.png")
                .with_transform(player_transform((0, 0).into())),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data.input.is_key(17, InputState::Pressed) {
            self.player_pos.y += 1;
            self.player_sprite.set_transform(player_transform(self.player_pos));
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.player_sprite.render_to(&mut frame);
    }
}

fn player_transform(pos: UVec2) -> Transform {
    let origin = Vec3::new(-1.0 + 0.125, -1.0 + 0.125, 0.0);
    Transform {
        translation: origin + (pos.extend(0).as_vec3() * 0.25),
        scale: (0.125, 0.125).into(),
        ..Default::default()
    }
}
