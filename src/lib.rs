pub mod audio;
pub mod graphics;
pub mod input;
pub mod sprite;

use std::time::{Duration, Instant};

use glam::UVec2;
use graphics::{Frame, Graphics};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

use crate::input::Input;

/// Variables used to initialise the game.
pub struct GameInit {
    pub window_resizable: bool,
    pub window_size: UVec2,
    pub window_title: &'static str,
    pub window_fullscreen: bool,
}

impl Default for GameInit {
    fn default() -> Self {
        Self {
            window_resizable: true,
            window_title: "Current window",
            window_size: UVec2::new(640, 480),
            window_fullscreen: false,
        }
    }
}

/// A struct containing references to all core components of the library.
pub struct GameData<'a> {
    pub graphics: &'a mut Graphics,
    pub input: &'a Input,
    /// The time since the last update.
    pub delta_time: Duration,
    pub window: &'a mut Window,
}

impl GameData<'_> {
    pub fn resize_window(&mut self, size: UVec2) {
        let size = PhysicalSize::new(size.x, size.y);
        self.window.set_inner_size(size);
        self.graphics.resize(size);
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
}

pub trait GameExt
where
    Self: Sized,
{
    fn run(init: GameInit) -> !;
}

impl<T: 'static> GameExt for T
where
    T: Game,
{
    /// Used to start the game.
    fn run(init: GameInit) -> ! {
        let event_loop = EventLoop::new();
        let mut window = WindowBuilder::new()
            .with_title(init.window_title)
            .with_inner_size(PhysicalSize::new(init.window_size.x, init.window_size.y))
            .with_resizable(init.window_resizable)
            .with_fullscreen(match init.window_fullscreen {
                true => Some(Fullscreen::Borderless(None)),
                false => None,
            })
            .build(&event_loop)
            .unwrap();

        let mut graphics = pollster::block_on(Graphics::new(&window));
        let mut input = Input::new();

        let mut game_data = GameData {
            graphics: &mut graphics,
            input: &input,
            delta_time: Duration::from_secs(0),
            window: &mut window,
        };
        let mut game = Self::init(&mut game_data);

        let mut last_update = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            let mut game_data = GameData {
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
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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
