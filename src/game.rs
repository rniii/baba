use std::marker::PhantomData;
use std::time::{Duration, Instant};

use sdl2::video::{DisplayMode, Window};

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
        let (mut window, mode) = self.init_window(&sdl)?;

        self.running = true;
        self.init_canvas(&window)?;

        let frame_limit = match self.settings.framerate {
            Framerate::Multiplier(mul) => {
                let base = mode.refresh_rate as f32;
                let base = if base > 0. { base } else { 60. };
                Duration::from_secs_f32(1. / (mul * base))
            }
            Framerate::Exact(fps) => Duration::from_secs_f32(1. / fps as f32),
            Framerate::Unlimited => Duration::ZERO,
        };
        let mut frame_start = Instant::now();
        let mut state = init();

        window.show();

        while self.running {
            self.process_events();

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
            },
        );

        Ok(sdl)
    }

    fn init_window(&self, sdl: &sdl2::Sdl) -> Result<(Window, DisplayMode)> {
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

        Ok((window, m))
    }

    fn init_canvas(&self, window: &Window) -> Result<()> {
        let canvas = window.clone().into_canvas().accelerated();
        let canvas = if self.settings.vsync {
            canvas.present_vsync()
        } else {
            canvas
        };
        let canvas = canvas.build().unwrap();
        log::info!("Using {} renderer", canvas.info().name);

        gfx::CANVAS.set(Some(canvas));

        Ok(())
    }

    fn process_events(&mut self) {
        unsafe {
            loop {
                let mut event = std::mem::MaybeUninit::uninit();
                if sdl2_sys::SDL_PollEvent(event.as_mut_ptr()) == 0 {
                    break;
                }
                let event = event.assume_init();
                match std::mem::transmute::<u32, sdl2_sys::SDL_EventType>(event.type_) {
                    sdl2_sys::SDL_EventType::SDL_QUIT => self.running = false,
                    sdl2_sys::SDL_EventType::SDL_KEYDOWN if event.key.repeat == 0 => {
                        let key = bytemuck::checked::cast(event.key.keysym.scancode as u32);
                        input::press_key(key);
                    }
                    sdl2_sys::SDL_EventType::SDL_KEYUP => {
                        let key = bytemuck::checked::cast(event.key.keysym.scancode as u32);
                        input::release_key(key);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Global engine settings.
pub struct Settings {
    /// Texture scaling mode, may be overriden with [`TextureOptions`][crate::TextureOptions].
    /// Defaults to [`ScaleMode::Nearest`].
    pub scale_mode: ScaleMode,
    /// Framerate limit for `update` calls. Default 1x the display's refresh rate.
    pub framerate: Framerate,
    /// Enable vertical sync (default off). Reduces tearing at the cost of some latency. You likely
    /// also want to set [framerate][Settings::framerate] if you use this.
    pub vsync: bool,
    /// Window creation settings.
    pub window: WindowSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_mode: ScaleMode::Nearest,
            framerate: Framerate::Multiplier(1.),
            vsync: false,
            window: WindowSettings::default(),
        }
    }
}

/// Framerate limit.
pub enum Framerate {
    /// Sets framerate to a multiple of the current display's refresh rate.
    Multiplier(f32),
    /// Sets framerate to an exact value.
    Exact(u32),
    /// No limits. Use this when you are going to implement your own limiting.
    Unlimited,
}

/// Window settings.
pub struct WindowSettings {
    /// Window title. Defaults to the name given to [`game`] or [`run`].
    pub title: Option<String>,
    /// Window size. Defaults to 800x600.
    pub size: (u32, u32),
    /// Allow window to be resized. Defaults to true.
    pub resizable: bool,
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
