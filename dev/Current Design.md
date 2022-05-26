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
    Empty.run();
}

struct Empty;

impl Game for Empty {}
```

### Render a rectangle

```rust
use current::*;
use current::sprite::Sprite;

fn main() {
    RectangleExample.run();
}

struct RectangleExample {
    sprite: Sprite,
}

impl Game for RectangleExample {
	fn init(data: &mut GameData) -> Self {
        let translation = Transfo::scale(0.5, 0.5).with_translation(0.5, 0.0);
        self.sprite = Sprite::new_image(&data.graphics, "graphics/test.png").with_transform(transform);
	}
    
    fn render(&self, data: &mut GameData) {
        self.sprite.render(data);
    }
}
```

