use kira::manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings};

pub struct Audio {
    pub manager: AudioManager,
}

impl Audio {
    pub(crate) fn new() -> Option<Self> {
        match AudioManager::<CpalBackend>::new(AudioManagerSettings::default()) {
            Ok(manager) => Some(Self { manager }),
            Err(err) => {
                eprintln!("Error creating audio handler:\n{:?}", err);
                None
            }
        }
    }
}
