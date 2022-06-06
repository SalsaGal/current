pub mod graphics;
pub mod input;
pub mod sprite;

use std::time::{Duration, Instant};

use glam::UVec2;
use graphics::{Frame, Graphics};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::input::Input;

/// Variables used to initialise the game.
pub struct GameInit {
    pub window_title: &'static str,
    pub window_size: UVec2,
}

impl Default for GameInit {
    fn default() -> Self {
        Self {
            window_title: "Current window",
            window_size: UVec2::new(640, 480),
        }
    }
}

/// A struct containing references to all core components of the library.
pub struct GameData<'a> {
    pub graphics: &'a mut Graphics,
    pub input: &'a Input,
    /// The time since the last update.
    pub delta_time: Duration,
}

pub trait Game: GameExt {
    /// Called once `GameExt::run()` is called. Used to create the game's struct
    /// while allowing access to the `GameData`.
    fn init(_: &mut GameData) -> Self;
    /// Called every frame. Has access to less parts of the library than Game::update
    /// however this has access to a `Frame` which contains things specific
    /// to rendering.
    fn render<'a>(&'a mut self, _: Frame<'a>) {}
    fn update(&mut self, _: &mut GameData) {}
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
        let window = WindowBuilder::new()
            .with_title(init.window_title)
            .with_inner_size(PhysicalSize::new(init.window_size.x, init.window_size.y))
            .build(&event_loop)
            .unwrap();

        let mut graphics = pollster::block_on(Graphics::new(&window));
        let mut input = Input::new();

        let mut game_data = GameData {
            graphics: &mut graphics,
            input: &input,
            delta_time: Duration::from_secs(0),
        };
        let mut game = Self::init(&mut game_data);

        let mut last_update = Instant::now();
        event_loop.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                let mut game_data = GameData {
                    graphics: &mut graphics,
                    input: &input,
                    delta_time: Instant::now() - last_update,
                };
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
                WindowEvent::MouseInput { button, state, .. } => input.handle_button(button, state),
                WindowEvent::Resized(size) => graphics.resize(size),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    graphics.resize(*new_inner_size)
                }
                _ => {}
            },
            _ => {}
        })
    }
}
