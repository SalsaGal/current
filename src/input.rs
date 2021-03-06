use std::collections::HashMap;

use glam::Vec2;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput, MouseButton, ScanCode};

pub struct Input {
    keys: HashMap<ScanCode, InputState>,
    buttons: HashMap<MouseButton, InputState>,
    /// The position of the mouse on the window.
    pub mouse_pos: Vec2,
    /// Amount of motion this update.
    pub mouse_mov: Vec2,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keys: HashMap::new(),
            buttons: HashMap::new(),
            mouse_pos: Vec2::ZERO,
            mouse_mov: Vec2::ZERO,
        }
    }

    /// Check if a mouse button is pressed.
    pub fn is_button(&self, button: MouseButton, state: InputState) -> bool {
        let actual = *self.buttons.get(&button).unwrap_or(&InputState::Up);
        match state {
            InputState::Down => actual == state || actual == InputState::Pressed,
            InputState::Up => actual == state || actual == InputState::Released,
            _ => actual == state,
        }
    }

    /// Check if a keyboard key is pressed.
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
        self.buttons
            .retain(|_, state| *state != InputState::Released);

        self.mouse_mov = Vec2::ZERO;
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

    pub(crate) fn handle_cursor(&mut self, pos: PhysicalPosition<f64>) {
        let old_pos = self.mouse_pos;
        self.mouse_pos = Vec2::new(pos.x as f32, pos.y as f32);
        self.mouse_mov = self.mouse_pos - old_pos;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputState {
    Up,
    Pressed,
    Down,
    Released,
}
