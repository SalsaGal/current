use current::*;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

fn main() {
    AudioExample::run(GameInit::default());
}

struct AudioExample {
    sound: StaticSoundData,
}

impl Game for AudioExample {
    fn init(_: &mut GameData) -> Self {
        let sound = StaticSoundData::from_file("examples/test.ogg", StaticSoundSettings::default())
            .unwrap();

        Self { sound }
    }

    fn update(&mut self, data: &mut GameData) {
        if data
            .input
            .is_button(winit::event::MouseButton::Left, input::InputState::Pressed)
        {
            data.audio.manager.play(self.sound.clone()).unwrap();
        }
    }
}
