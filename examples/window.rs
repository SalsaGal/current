use current::*;

fn main() {
    Window::run();
}

struct Window;

impl Game for Window {
    fn init(_: &mut GameData) -> Self {
        Self
    }
}
