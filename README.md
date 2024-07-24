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

## Roadmap?

- [x] Primitives rendering
    - [x] Public `Drawable` api
    - [x] Public `Canvas` api
- [ ] Shape rendering
    - SDL apis? maybe use `epaint`? both?
- [ ] Text rendering (SDL_ttf)
- [ ] Event alternative to `input::is_key_down` etc
- [ ] Audio playback (SDL_audio)
- [ ] Config loading, also more engine settings
- [ ] Document all APIs
- [ ] Write the Baba Engine Book
