use current::graphics::Frame;
use current::sprite::{Corner, Horizontal, Sprite, Transform, Vertical};
use current::*;
use glam::{Vec2, Vec3};
use wgpu::Color;

fn main() {
    CornerTest::run(GameInit {
        window_size: (600, 500).into(),
        ..Default::default()
    });
}

struct CornerTest {
    rect: Sprite,
    i: u8,
}

fn corner(i: u8) -> Corner {
    match i % 4 {
        0 => Corner {
            h: Horizontal::Left,
            v: Vertical::Up,
        },
        1 => Corner {
            h: Horizontal::Right,
            v: Vertical::Up,
        },
        2 => Corner {
            h: Horizontal::Left,
            v: Vertical::Down,
        },
        3 => Corner {
            h: Horizontal::Right,
            v: Vertical::Down,
        },
        _ => unreachable!(),
    }
}

impl Game for CornerTest {
    fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics, Color::GREEN).with_transform(
                Transform {
                    scale: Vec2::new(200.0, 200.0),
                    ..Default::default()
                }
                .with_translation_corner(
                    Vec3::ZERO,
                    Corner {
                        h: Horizontal::Left,
                        v: Vertical::Up,
                    },
                ),
            ),
            i: 0,
        }
    }

    fn update(&mut self, data: &mut GameData) {
        if data
            .input
            .is_button(winit::event::MouseButton::Left, input::InputState::Pressed)
        {
            self.i += 1;
            self.rect.set_transform(
                Transform {
                    scale: Vec2::new(200.0, 200.0),
                    ..Default::default()
                }
                .with_translation_corner(Vec3::ZERO, corner(self.i)),
            );
        }
    }

    fn render<'a>(&'a mut self, mut frame: Frame<'a>) {
        self.rect.render_to(&mut frame);
    }
}
