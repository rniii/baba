# baba

Extremely simple library for game development, inspired by love2d and raylib.

```rs
fn main() -> baba::Result {
    baba::run("My game", MyGame::update)
}

#[derive(Default)]
struct MyGame;

impl MyGame {
    fn update(&mut self) {
        gfx::clear(Color::WHITE);
        // draw stuff..
    }
}
```
