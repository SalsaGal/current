use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;

pub struct GameData {
}

pub trait Game : GameExt {
    fn init(_: &mut GameData) -> Self;
    fn update(&mut self, _: &mut GameData) {}
}

pub trait GameExt where Self: Sized {
    fn run() -> !;
}

macro_rules! make_game_data {
    () => {
        GameData {
        }
    };
}

impl<T: 'static> GameExt for T where T: Game {
    fn run() -> ! {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut game_data = make_game_data!();
        let mut game = Self::init(&mut game_data);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    let mut game_data = make_game_data!();
                    game.update(&mut game_data);
                    window.request_redraw();
                },
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        _ => {},
                    }
                },
                _ => {},
            }
        })
    }
}
