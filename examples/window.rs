use current::*;

fn main() {
    Window::run();
}

struct Window;

impl Game for Window {
    fn init(data: &mut GameData) -> Self {
        data.set_title("Window Title");
        Self
    }
}
