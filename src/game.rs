use std::marker::PhantomData;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::{gfx, input, Result, ScaleMode, SdlError};

pub fn game<State>(
    name: impl Into<String>,
    update: impl Fn(&mut State),
) -> Game<State, impl Fn(&mut State)> {
    Game {
        name: name.into(),
        update,
        settings: Settings::default(),
        running: false,
        _state: PhantomData,
    }
}

pub fn run<State: Default>(name: impl Into<String>, update: impl Fn(&mut State)) -> Result {
    game(name, update).run()
}

pub struct Game<State, Update: Fn(&mut State)> {
    name: String,
    update: Update,
    settings: Settings,
    running: bool,
    _state: PhantomData<State>,
}

#[allow(clippy::missing_const_for_fn)] // Game can't be const initialised
impl<State, Update: Fn(&mut State)> Game<State, Update> {
    #[must_use]
    pub fn scale_mode(mut self, scale_mode: ScaleMode) -> Self {
        self.settings.scale_mode = scale_mode;
        self
    }

    #[must_use]
    pub fn window_settings(mut self, settings: WindowSettings) -> Self {
        self.settings.window = settings;
        self
    }

    #[must_use]
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.settings.window.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.settings.window.size = (width, height);
        self
    }

    pub fn run(self) -> Result
    where
        State: Default,
    {
        self.run_with(State::default)
    }

    pub fn run_with(mut self, init: impl FnOnce() -> State) -> Result {
        env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
            .format_timestamp_millis()
            .init();

        let sdl = self.init_sdl()?;
        let mut window = self.init_window(&sdl)?;
        let mut pump = sdl.event_pump().map_err(SdlError)?;

        self.running = true;
        self.init_canvas(&window)?;

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

    fn init_sdl(&self) -> Result<sdl2::Sdl> {
        let sdl = sdl2::init().map_err(SdlError)?;
        sdl2::hint::set("SDL_APP_NAME", &self.name);
        // sdl2::hint::set("SDL_IME_SUPPORT_EXTENDED_TEXT", "1");
        sdl2::hint::set("SDL_VIDEO_DOUBLE_BUFFER", "1");
        sdl2::hint::set(
            "SDL_RENDER_SCALE_QUALITY",
            match self.settings.scale_mode {
                ScaleMode::Nearest => "0",
                ScaleMode::Linear => "1",
                ScaleMode::Anisotropic => "2",
            },
        );

        Ok(sdl)
    }

    fn init_window(&self, sdl: &sdl2::Sdl) -> Result<sdl2::video::Window> {
        let settings = &self.settings.window;
        let video = sdl.video().map_err(SdlError)?;

        let mut flags = 0;
        flags |= sdl2_sys::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        if settings.resizable {
            flags |= sdl2_sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
        }

        let title = settings.title.as_ref().unwrap_or(&self.name);
        let (width, height) = settings.size;
        let window = video
            .window(title, width, height)
            .set_window_flags(flags)
            .build()
            .unwrap();

        let m = window.display_mode().unwrap();
        log::info!("Created window {}x{} @{}fps", m.w, m.h, m.refresh_rate);

        Ok(window)
    }

    fn init_canvas(&self, window: &Window) -> Result<()> {
        let canvas = window.clone().into_canvas().accelerated().build().unwrap();
        log::info!("Using {} renderer", canvas.info().name);

        gfx::CANVAS.set(Some(canvas));

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

/// Global engine settings.
pub struct Settings {
    /// Texture scaling mode, may be overriden with [`TextureOptions`][crate::TextureOptions].
    /// Defaults to [`ScaleMode::Nearest`].
    pub scale_mode: ScaleMode,
    /// Window creation settings.
    pub window: WindowSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_mode: ScaleMode::Nearest,
            window: WindowSettings::default(),
        }
    }
}

/// Window settings.
pub struct WindowSettings {
    /// Window title. Defaults to the name given to [`game`] or [`run`].
    pub title: Option<String>,
    /// Window size. Defaults to 800x600.
    pub size: (u32, u32),
    /// Allow window to be resized. Defaults to true.
    pub resizable: bool,
    // pub position
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: None,
            size: (800, 600),
            resizable: true,
        }
    }
}
