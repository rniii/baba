use baba::prelude::*;

fn main() -> baba::Result {
    baba::game("SOKOBAN", Soko::update)
        .viewport(Viewport::new(32 * 7, 32 * 7))
        .run_with(Soko::new)
}

const MAP: &str = "\
####..
#.O#..
#..###
#@P..#
#..*.#
#..###
####..";

struct Entity {
    texture: TextureSlice,
    position: Vec2,
}

impl Entity {
    fn new(texture: TextureSlice, position: Vec2) -> Self {
        Self { texture, position }
    }
}

struct Soko {
    player: Entity,
    objects: Vec<Entity>,
    targets: Vec<Entity>,
    walls: Vec<Entity>,

    won: bool,
}

impl Soko {
    fn new() -> Self {
        let tiles = Texture::load("examples/tiles.png");

        let object_slice = tiles.slice(Rect::new(0, 9, 8, 8));
        let target_slice = tiles.slice(Rect::new(9, 9, 8, 8));
        let player_slice = tiles.slice(Rect::new(27, 9, 8, 8));
        let mut wall_slices = Vec::new();
        for x in 0..4 {
            wall_slices.push(tiles.slice(Rect::new(x * 9, 0, 8, 8)));
        }

        let mut player = Entity::new(player_slice, vec2(0., 0.));
        let mut objects = Vec::new();
        let mut targets = Vec::new();
        let mut walls = Vec::new();
        let map = Vec::from_iter(MAP.split('\n').map(|line| Vec::from_iter(line.chars())));

        for (y, line) in map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let position = vec2(x as f32, y as f32);
                match tile {
                    'O' => targets.push(Entity::new(target_slice.clone(), position)),
                    '*' => objects.push(Entity::new(object_slice.clone(), position)),
                    '@' => {
                        targets.push(Entity::new(target_slice.clone(), position));
                        objects.push(Entity::new(object_slice.clone(), position));
                    }
                    'P' => player.position = position,
                    '#' => {
                        let left = map[y].get(x.wrapping_sub(1)) == Some(&'#');
                        let right = map[y].get(x + 1) == Some(&'#');
                        let i = left as usize * 2 + right as usize;

                        walls.push(Entity::new(wall_slices[i].clone(), position));
                    }
                    _ => continue,
                }
            }
        }

        Self {
            player,
            objects,
            targets,
            walls,
            won: false,
        }
    }

    fn update(&mut self) {
        if is_key_pressed(KeyCode::R) {
            *self = Self::new();
            return;
        }

        let mut movement = Vec2::ZERO;
        if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            movement.x -= 1.;
        }
        if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            movement.x += 1.;
        }
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            movement.y -= 1.;
        }
        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            movement.y += 1.;
        }

        let mut collided = false;
        let position = self.player.position + movement;
        if self.walls.iter().any(|e| e.position == position) {
            collided = true;
        } else if let Some(obj) = self.objects.iter().position(|e| e.position == position) {
            let position = position + movement;
            if self.solids().any(|e| e.position == position) {
                collided = true;
            } else {
                self.objects[obj].position = position;
            }
        }

        if !collided {
            self.player.position = position;
        }

        if !self.won
            && self
                .objects
                .iter()
                .all(|object| self.targets.iter().any(|e| e.position == object.position))
        {
            self.won = true;
            // you are free!
            self.walls.clear();
        }

        self.draw();
    }

    fn draw(&self) {
        gfx::clear(Color::from_rgb(0x2f, 0x28, 0x43));

        for entity in self
            .targets
            .iter()
            .chain(self.objects.iter())
            .chain(self.walls.iter())
            .chain(std::iter::once(&self.player))
        {
            gfx::draw(&entity.texture, (entity.position * 32.0, (4., 4.)));
        }
    }

    fn solids(&self) -> impl Iterator<Item = &Entity> {
        self.walls.iter().chain(self.objects.iter())
    }
}
