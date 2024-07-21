use std::marker::PhantomData;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::EventPump;

use crate::{gfx, input, Result, SdlError};

pub fn game<State>(
    name: impl Into<String>,
    update: impl Fn(&mut State),
) -> Game<State, impl Fn(&mut State)> {
    let name = name.into();
    Game {
        name: name.clone(),
        update,
        settings: Settings {
            title: name,
            ..Default::default()
        },
        running: false,
        _state: PhantomData,
    }
}

pub struct Game<State, Update: Fn(&mut State)> {
    name: String,
    update: Update,
    settings: Settings,
    running: bool,
    _state: PhantomData<State>,
}

impl<State, Update: Fn(&mut State)> Game<State, Update> {
    pub fn window_size(self, width: u32, height: u32) -> Self {
        Self {
            settings: Settings {
                width,
                height,
                ..self.settings
            },
            ..self
        }
    }

    pub fn run(self) -> Result
    where
        State: Default,
    {
        self.run_with(State::default)
    }

    pub fn run_with(mut self, init: impl FnOnce() -> State) -> Result {
        self.running = true;

        let sdl = sdl2::init().map_err(SdlError)?;
        sdl2::hint::set("SDL_APP_NAME", &self.name);
        // sdl2::hint::set("SDL_IME_SUPPORT_EXTENDED_TEXT", "1");
        sdl2::hint::set("SDL_VIDEO_DOUBLE_BUFFER", "1");

        let video = sdl.video().map_err(SdlError)?;

        let mut builder = video.window(
            &self.settings.title,
            self.settings.width,
            self.settings.height,
        );
        builder.resizable();
        builder.hidden();

        let mut window = builder.build().unwrap();
        let mut pump = sdl.event_pump().map_err(SdlError)?;

        let canvas = window.clone().into_canvas().accelerated().build().unwrap();
        gfx::CANVAS.replace(Some(canvas));

        let frame_limit = Duration::from_secs_f64(1.0 / 60.0);
        let mut frame_start = Instant::now();
        let mut state = init();

        window.show();

        while self.running {
            self.process_events(&mut pump);

            (self.update)(&mut state);

            gfx::display();
            input::clear();

            let now = Instant::now();
            let dt = now - std::mem::replace(&mut frame_start, now);
            if dt <= frame_limit {
                std::thread::sleep(frame_limit - dt);
            }
        }

        Ok(())
    }

    fn process_events(&mut self, pump: &mut EventPump) {
        while let Some(event) = pump.poll_event() {
            match event {
                Event::Quit { .. } => self.running = false,
                Event::KeyDown {
                    scancode, repeat, ..
                } if !repeat => {
                    input::press_key(scancode.unwrap());
                }
                Event::KeyUp { scancode, .. } => {
                    input::release_key(scancode.unwrap());
                }
                _ => {}
            }
        }
    }
}

pub struct Settings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            title: String::new(),
            width: 800,
            height: 600,
        }
    }
}
