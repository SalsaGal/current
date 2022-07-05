use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};

pub struct Audio {
    pub manager: AudioManager,
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
        }
    }
}
