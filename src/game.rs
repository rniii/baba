use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::{gfx, Result, SdlError};

pub fn game<State>(
    title: impl AsRef<str>,
    update: impl Fn(&mut State),
) -> Game<State, impl Fn(&mut State)> {
    Game {
        update,
        settings: Settings {
            title: title.as_ref().to_owned(),
            ..Default::default()
        },
        running: false,
        _state: PhantomData,
    }
}

pub struct Game<State, Update: Fn(&mut State)> {
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
            for event in pump.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    self.running = false;
                }
            }

            (self.update)(&mut state);
            gfx::with_canvas(|canvas| canvas.present());

            let now = Instant::now();
            let dt = now - std::mem::replace(&mut frame_start, now);
            if dt <= frame_limit {
                std::thread::sleep(frame_limit - dt);
            }
        }

        Ok(())
    }
}

pub struct Settings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            title: String::new(),
            width: 800,
            height: 600,
            vsync: false,
        }
    }
}
