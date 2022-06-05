# Current Design

Current is a 2D Rust game library, built on WGPU. It aims to simplify as much as it can, while still allowing access to the more complicated parts of the code.

## Important Types

### GameData

The `GameData` struct will be accessable by the Game in every function. It is a list of mutable references to all the other important structs in the library. Here is a list of everything it includes:

- Graphics handler
- Input handler
- Optional render pass (optional because it is only accessible in `Game::render`)

## Example Code

### Empty window

```rust
use current::*;

fn main() {
    Window::run(GameInit::default());
}

struct Window;

impl Game for Window {
    fn init(_: &mut GameData) -> Self {
        Self
    }
}
```

### Render a rectangle

```rust
use current::*;
use current::graphics::Frame;
use current::sprite::Sprite;

fn main() {
    Rect::run();
}

struct Rect {
    rect: Sprite,
}

impl Game for Rect {
	fn init(data: &mut GameData) -> Self {
        Self {
            rect: Sprite::new_color_rect(data.graphics),
        }
	}
    
    fn render<'a>(&'a self, mut frame: Frame<'a>) {
        self.rect.render(&mut frame);
    }
}
```
