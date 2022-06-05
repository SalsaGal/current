use current::sprite::{Sprite, Transform};
use current::*;
use glam::{Vec2, Vec3};
use wgpu::Color;

fn main() {
    Platformer::run();
}

struct Platformer {
    player: Player,
}

impl Game for Platformer {
    fn init(data: &mut GameData) -> Self {
        Self {
            player: Player::new(data),
        }
    }

    fn update(&mut self, data: &mut GameData) {
        let delta_time = data.delta_time.as_secs_f32();
        self.player.update(delta_time);
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        self.player.update_sprite();
        self.player.sprite.render_to(&mut frame);
    }
}

struct Player {
    pos: Vec2,
    mov: Vec2,
    sprite: Sprite,
}

impl Player {
    fn new(data: &mut GameData) -> Self {
        let sprite = Sprite::new_color_rect(data.graphics, Color::RED).with_transform(
            Transform::default()
                .with_scale(Vec2::new(1.0 / 16.0, 1.0 / 16.0))
                .with_translation_centered(Vec3::ZERO),
        );

        Self {
            pos: Vec2::ZERO,
            mov: Vec2::ZERO,
            sprite,
        }
    }

    fn update(&mut self, delta_time: f32) {
        const GRAVITY: f32 = 1.0;
        self.mov.y -= GRAVITY * delta_time;

        self.pos += self.mov;
    }

    fn update_sprite(&mut self) {
        let mut transform = self.sprite.transform;
        transform = transform.with_translation_centered(self.pos.extend(0.0));
        self.sprite.set_transform(transform);
    }
}
