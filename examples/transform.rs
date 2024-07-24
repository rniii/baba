use baba::prelude::*;

fn main() -> baba::Result {
    baba::run("Example", State::update)
}

struct State {
    position: Vec2,
    scale: Vec2,
    angle: f32,
    creature: TextureSlice,
}

impl Default for State {
    fn default() -> Self {
        Self {
            position: vec2(800., 600.) / 2., // window center
            scale: Vec2::splat(8.),
            angle: 0.,
            creature: Texture::load_with("examples/tiles.png", Origin::CENTER)
                .slice(Rect::new(27, 9, 8, 8)),
        }
    }
}

impl State {
    fn update(&mut self) {
        gfx::clear(Color::from_rgb(0x2f, 0x28, 0x43));

        if is_key_down(KeyCode::A) {
            self.position.x -= 1.;
        }
        if is_key_down(KeyCode::D) {
            self.position.x += 1.;
        }
        if is_key_down(KeyCode::W) {
            self.position.y -= 1.;
        }
        if is_key_down(KeyCode::S) {
            self.position.y += 1.;
        }

        if is_key_down(KeyCode::Q) {
            self.angle = (self.angle - 0.04) % TAU;
        }
        if is_key_down(KeyCode::E) {
            self.angle = (self.angle + 0.04) % TAU;
        }

        if is_key_down(KeyCode::J) {
            self.scale /= 1.005;
        }
        if is_key_down(KeyCode::K) {
            self.scale *= 1.005;
        }

        gfx::draw(&self.creature, (self.position, self.scale, self.angle));
    }
}
