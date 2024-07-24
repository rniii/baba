use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::{gfx, input, Result, ScaleMode, Viewport};

pub struct Game<State, Update> {
    name: String,
    update: Update,
    settings: Settings,
    window: WindowSettings,
    _state: PhantomData<State>,
}

#[allow(clippy::missing_const_for_fn)] // Game can't be const initialised
impl<State, Update: Fn(&mut State)> Game<State, Update> {
    pub fn new(name: String, update: Update) -> Self {
        Self {
            name,
            update,
            settings: Settings::default(),
            window: WindowSettings::default(),
            _state: PhantomData,
        }
    }

    #[must_use]
    pub fn settings(mut self, settings: Settings) -> Self {
        self.settings = settings;
        self
    }

    #[must_use]
    pub fn vsync(mut self) -> Self {
        self.settings.vsync = true;
        self
    }

    #[must_use]
    pub fn framerate(mut self, framerate: Framerate) -> Self {
        self.settings.framerate = framerate;
        self
    }

    #[must_use]
    pub fn viewport(mut self, viewport: Viewport) -> Self {
        self.settings.viewport = Some(viewport);
        self
    }

    #[must_use]
    pub fn scale_mode(mut self, scale_mode: ScaleMode) -> Self {
        self.settings.scale_mode = scale_mode;
        self
    }

    #[must_use]
    pub fn window(mut self, settings: WindowSettings) -> Self {
        self.window = settings;
        self
    }

    #[must_use]
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window.size = (width, height);
        self
    }

    pub fn run(self) -> Result
    where
        State: Default,
    {
        self.run_with(State::default)
    }

    pub fn run_with(self, init: impl FnOnce() -> State) -> Result {
        env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
            .format_timestamp_millis()
            .init();

        let (canvas, mode) = self.init_canvas()?;

        let frame_limit = match self.settings.framerate {
            Framerate::Multiplier(mul) => {
                let base = mode.refresh as f32;
                let base = if base > 0. { base } else { 60. };
                Duration::from_secs_f32(1. / (mul * base))
            }
            Framerate::Exact(fps) => Duration::from_secs_f32(1. / fps as f32),
            Framerate::Unlimited => Duration::ZERO,
        };
        let mut frame_start = Instant::now();
        let mut state = init();

        canvas.show_window();

        while canvas.process_events() {
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

    fn init_canvas(&self) -> Result<(gfx::Canvas, gfx::DisplayMode)> {
        let sdl = sdl2::init().unwrap();
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

        let mut flags = 0;
        flags |= sdl2_sys::SDL_WindowFlags::SDL_WINDOW_HIDDEN as u32;
        if self.window.resizable {
            flags |= sdl2_sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;
        }

        let mut canvas = gfx::Canvas::new(&sdl.video().unwrap(), flags)?;
        canvas.set_window_title(self.window.title.as_ref().unwrap_or(&self.name));
        canvas.set_window_size(self.window.size.0, self.window.size.1);

        if let Some(viewport) = &self.settings.viewport {
            canvas.set_viewport(viewport);
        }

        if self.settings.vsync && !canvas.set_vsync(true) {
            log::warn!("Failed to set vsync!")
        }

        let mode = canvas.get_display_mode();
        log::info!(
            "Created window {}x{} @{}fps",
            mode.width,
            mode.height,
            mode.refresh
        );
        log::info!("Using {} renderer", mode.renderer);

        gfx::CANVAS.set(Some(canvas.clone()));

        Ok((canvas, mode))
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
    /// Viewport. If this is set, it will map coordinates to fit it's size, instead of following
    /// window coordinates.
    pub viewport: Option<Viewport>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_mode: ScaleMode::Nearest,
            framerate: Framerate::Multiplier(1.),
            vsync: false,
            // XXX: could have default?
            viewport: None,
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
