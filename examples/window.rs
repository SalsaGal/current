use current::*;

fn main() {
    Window::run(GameInit {
        window_title: "Window Title",
        ..Default::default()
    });
}

struct Window;

impl Game for Window {
    fn init(_: &mut GameData) -> Self {
        Self
    }
}
