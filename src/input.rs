use std::collections::HashMap;

use winit::event::{KeyboardInput, MouseButton, ScanCode, ElementState};

pub struct Input {
    keys: HashMap<ScanCode, InputState>,
    buttons: HashMap<MouseButton, InputState>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keys: HashMap::new(),
            buttons: HashMap::new(),
        }
    }

    pub fn is_button(&self, button: MouseButton, state: InputState) -> bool {
        let actual = *self.buttons.get(&button).unwrap_or(&InputState::Up);
        match state {
            InputState::Down => actual == state || actual == InputState::Pressed,
            InputState::Up => actual == state || actual == InputState::Released,
            _ => actual == state,
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

        self.buttons.iter_mut().for_each(|(_, state)| match state {
            InputState::Pressed => *state = InputState::Down,
            InputState::Released => *state = InputState::Up,
            _ => {}
        });
        self.buttons.retain(|_, state| *state != InputState::Released);
    }

    pub(crate) fn handle_key(&mut self, input: KeyboardInput) {
        match input.state {
            ElementState::Pressed => {
                if self.keys.get(&input.scancode) != Some(&InputState::Down) {
                    self.keys.insert(input.scancode, InputState::Pressed);
                }
            }
            ElementState::Released => {
                self.keys.insert(input.scancode, InputState::Released);
            }
        }
    }

    pub(crate) fn handle_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if self.buttons.get(&button) != Some(&InputState::Down) {
                    self.buttons.insert(button, InputState::Pressed);
                }
            }
            ElementState::Released => {
                self.buttons.insert(button, InputState::Released);
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
