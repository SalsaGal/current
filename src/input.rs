use std::collections::HashMap;

use winit::event::{KeyboardInput, ScanCode};

pub struct Input {
    keys: HashMap<ScanCode, InputState>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn is_key(&self, key: ScanCode, state: InputState) -> bool {
        let actual = *self.keys.get(&key).unwrap_or(&InputState::Up);
        match state {
            InputState::Down => actual == state || actual == InputState::Pressed,
            InputState::Up => actual == state || actual == InputState::Released,
            _ => actual == state,
        }
    }

    pub(crate) fn update(&mut self) {
        self.keys.iter_mut().for_each(|(_, state)| match state {
            InputState::Pressed => *state = InputState::Down,
            InputState::Released => *state = InputState::Up,
            _ => {}
        });
        self.keys.retain(|_, state| *state != InputState::Released);
    }

    pub(crate) fn handle(&mut self, input: KeyboardInput) {
        match input.state {
            winit::event::ElementState::Pressed => {
                if self.keys.get(&input.scancode) != Some(&InputState::Down) {
                    self.keys.insert(input.scancode, InputState::Pressed);
                }
            }
            winit::event::ElementState::Released => {
                self.keys.insert(input.scancode, InputState::Released);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputState {
    Up,
    Pressed,
    Down,
    Released,
}
