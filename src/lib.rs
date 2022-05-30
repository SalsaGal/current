pub mod graphics;
pub mod sprite;
pub mod input;

use std::time::{Duration, Instant};

use graphics::{Graphics, Frame};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;

use crate::input::Input;

pub struct GameData<'a> {
    pub graphics: &'a mut Graphics,
    pub input: &'a Input,
    pub delta_time: Duration,
}

pub trait Game : GameExt {
    fn init(_: &mut GameData) -> Self;
    fn render<'a>(&'a mut self, _: Frame<'a>) {}
    fn update(&mut self, _: &mut GameData) {}
}

pub trait GameExt where Self: Sized {
    fn run() -> !;
}

impl<T: 'static> GameExt for T where T: Game {
    fn run() -> ! {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut graphics = pollster::block_on(Graphics::new(&window));
        let mut input = Input::new();

        let mut game_data = GameData {
            graphics: &mut graphics,
            input: &input,
            delta_time: Duration::from_secs(0),
        };
        let mut game = Self::init(&mut game_data);

        let mut last_update = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            match event {
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
                },
                Event::RedrawRequested(..) => {
                    graphics.render(|pass| game.render(pass));
                },
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input: event, .. } => input.handle(event),
                        WindowEvent::Resized(size) => graphics.resize(size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => graphics.resize(*new_inner_size),
                        _ => {},
                    }
                },
                _ => {},
            }
        })
    }
}
