pub mod audio;
pub mod graphics;
pub mod input;
pub mod random;
pub mod sprite;

use std::time::{Duration, Instant};

use audio::Audio;
use glam::UVec2;
use graphics::{Frame, Graphics};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

use crate::input::Input;

/// A struct containing references to all core components of the library.
pub struct GameData<'a> {
    pub audio: &'a mut Audio,
    pub graphics: &'a mut Graphics,
    pub input: &'a Input,
    /// The time since the last update.
    pub delta_time: Duration,
    pub window: &'a mut Window,
}

impl GameData<'_> {
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.window.set_fullscreen(match fullscreen {
            true => Some(Fullscreen::Borderless(None)),
            false => None,
        });
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn set_window_size(&mut self, size: UVec2) {
        let size = PhysicalSize::new(size.x, size.y);
        self.window.set_inner_size(size);
        self.graphics.resize(size);
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
}

pub trait Game: GameExt {
    /// Called once `GameExt::run()` is called. Used to create the game's struct
    /// while allowing access to the `GameData`.
    fn init(_: &mut GameData) -> Self;
    /// Called every frame. Has access to less parts of the library than Game::update
    /// however this has access to a `Frame` which contains things specific
    /// to rendering.
    fn render<'a>(&'a mut self, _: Frame<'a>) {}
    /// Called repeatedly. Put your gameplay logic in here.
    fn update(&mut self, _: &mut GameData) {}
    /// Handle input events from winit directly before the engine handles them. Useful
    /// for more specific uses of hardware or features that Current doesn't support.
    fn handle_event(&mut self, _: &mut GameData, _: &Event<()>) {}
    /// Code executed when the game quits, this function returns a boolean of whether to
    /// actually quit or not.
    fn exit(&mut self, _: &mut GameData) -> bool {
        true
    }
}

pub trait GameExt
where
    Self: Sized,
{
    /// Begin executing the game.
    fn run() -> !;
}

impl<T: 'static> GameExt for T
where
    T: Game,
{
    /// Used to start the game.
    fn run() -> ! {
        let event_loop = EventLoop::new();
        let mut window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut audio = Audio::new();
        let mut graphics = pollster::block_on(Graphics::new(&window));
        let mut input = Input::new();

        let mut game_data = GameData {
            audio: &mut audio,
            graphics: &mut graphics,
            input: &input,
            delta_time: Duration::from_secs(0),
            window: &mut window,
        };
        let mut game = Self::init(&mut game_data);

        let mut last_update = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            let mut game_data = GameData {
                audio: &mut audio,
                graphics: &mut graphics,
                input: &input,
                delta_time: Instant::now() - last_update,
                window: &mut window,
            };

            game.handle_event(&mut game_data, &event);
            match event {
                Event::MainEventsCleared => {
                    game.update(&mut game_data);
                    input.update();
                    window.request_redraw();
                    last_update = Instant::now();
                }
                Event::RedrawRequested(..) => {
                    graphics.render(|pass| game.render(pass));
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        if game.exit(&mut game_data) {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => input.handle_cursor(position),
                    WindowEvent::KeyboardInput { input: event, .. } => input.handle_key(event),
                    WindowEvent::MouseInput { button, state, .. } => {
                        input.handle_button(button, state)
                    }
                    WindowEvent::Resized(size) => graphics.resize(size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        graphics.resize(*new_inner_size)
                    }
                    _ => {}
                },
                _ => {}
            }
        })
    }
}
