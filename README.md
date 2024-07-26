# baba

Extremely simple library for game development, inspired by love2d and raylib.

Its main goal is to provide a robust base for games of any complexity. It is currently built on top of SDL2, which
already has widespread usage and supports a huge variety of systems.

``` rs
fn main() -> baba::Result {
    baba::run("My game", MyGame::update)
}

impl MyGame {
    fn update(&mut self) {
        // Update your game logic and draw onto the screen!
        gfx::clear(Color::WHITE);
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
- [x] Document all APIs
- [ ] Write the Baba Engine Book
