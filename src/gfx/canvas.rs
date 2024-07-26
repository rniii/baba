use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use sdl2::VideoSubsystem;
use sdl2_sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_EventType, SDL_GetRendererInfo,
    SDL_GetWindowDisplayMode, SDL_PollEvent, SDL_RenderClear, SDL_RenderGeometry,
    SDL_RenderPresent, SDL_RenderSetIntegerScale, SDL_RenderSetLogicalSize, SDL_RenderSetVSync,
    SDL_Renderer, SDL_SetRenderDrawColor, SDL_SetWindowMinimumSize, SDL_SetWindowSize,
    SDL_SetWindowTitle, SDL_ShowWindow, SDL_Window, SDL_bool, SDL_WINDOWPOS_UNDEFINED_MASK,
};
use thiserror::Error;

use crate::gfx::{Drawable, Texture, Transform, Vertex};
use crate::{input, SdlError};

/// Defines how coordinates are translated.
#[must_use]
#[derive(Debug, Clone)]
pub struct Viewport {
    /// Maximum X and Y.
    pub logical_size: (u32, u32),
    /// How should coordinates be divided.
    pub scaling: ViewportScaling,
}

impl Viewport {
    /// Shorthand for `logical_size` and integer scaling.
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            logical_size: (width, height),
            scaling: ViewportScaling::Integer,
        }
    }

    /// Disables integer scaling.
    pub const fn fractional(mut self) -> Self {
        self.scaling = ViewportScaling::Fractional;
        self
    }

    /// Enables integer scaling.
    pub const fn integer(mut self) -> Self {
        self.scaling = ViewportScaling::Integer;
        self
    }
}

/// Toggles integer scaling for the viewport.
#[derive(Debug, Clone)]
pub enum ViewportScaling {
    /// Integer scaling. Coordinates will always use the smallest multiple of window coordinates.
    Integer,
    /// No integer scaling.
    Fractional,
}

/// Canvas creation error.
#[derive(Debug, Error)]
pub enum CanvasError {
    /// Backend error, this system might not be supported.
    #[error(transparent)]
    Sdl(#[from] SdlError),
}

/// An object responsible for rendering stuff onto a window.
#[derive(Clone)]
pub struct Canvas {
    window: NonNull<SDL_Window>,
    renderer: NonNull<SDL_Renderer>,
    _video: VideoSubsystem,
}

impl Canvas {
    pub(crate) fn new(video: &VideoSubsystem, flags: u32) -> Result<Self, CanvasError> {
        let position = SDL_WINDOWPOS_UNDEFINED_MASK as i32;

        let window = unsafe { SDL_CreateWindow(std::ptr::null(), position, position, 0, 0, flags) };
        let window = NonNull::new(window).ok_or_else(SdlError::from_sdl)?;

        let renderer = unsafe { SDL_CreateRenderer(window.as_ptr(), -1, 0) };
        let renderer = NonNull::new(renderer).ok_or_else(SdlError::from_sdl)?;

        Ok(Self {
            window,
            renderer,
            _video: video.clone(),
        })
    }

    pub(crate) fn renderer(&mut self) -> *mut SDL_Renderer {
        self.renderer.as_ptr()
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn process_events(&self) -> bool {
        let mut event = MaybeUninit::uninit();

        while unsafe { SDL_PollEvent(event.as_mut_ptr()) } == 1 {
            let event = unsafe { event.assume_init() };

            unsafe {
                match std::mem::transmute::<u32, SDL_EventType>(event.type_) {
                    SDL_EventType::SDL_QUIT => return false,
                    SDL_EventType::SDL_KEYDOWN if event.key.repeat == 0 => {
                        let key = bytemuck::checked::cast(event.key.keysym.scancode as u32);
                        input::press_key(key);
                    }
                    SDL_EventType::SDL_KEYUP => {
                        let key = bytemuck::checked::cast(event.key.keysym.scancode as u32);
                        input::release_key(key);
                    }
                    _ => {}
                }
            }
        }

        true
    }

    /// Queries some information about the window.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn get_display_mode(&self) -> DisplayMode {
        let mut mode = MaybeUninit::zeroed();
        let mut info = MaybeUninit::zeroed();
        if unsafe { SDL_GetRendererInfo(self.renderer.as_ptr(), info.as_mut_ptr()) } < 0 {
            log::warn!("Failed to query renderer: {}", SdlError::from_sdl());
        }

        if unsafe { SDL_GetWindowDisplayMode(self.window.as_ptr(), mode.as_mut_ptr()) } < 0 {
            log::warn!("Failed to query display: {}", SdlError::from_sdl());
        }

        let mode = unsafe { mode.assume_init() };
        let renderer = unsafe { info.assume_init() };
        let renderer = unsafe {
            if renderer.name.is_null() {
                ""
            } else {
                CStr::from_ptr(renderer.name).to_str().unwrap()
            }
        };

        DisplayMode {
            width: mode.w as u32,
            height: mode.h as u32,
            refresh: mode.refresh_rate as u32,
            renderer,
        }
    }

    /// Sets the window title.
    pub fn set_window_title(&mut self, title: &str) {
        unsafe { SDL_SetWindowTitle(self.window.as_ptr(), title.as_ptr().cast()) };
    }

    /// Sets the window size.
    pub fn set_window_size(&mut self, width: u32, height: u32) {
        unsafe { SDL_SetWindowSize(self.window.as_ptr(), width as i32, height as i32) };
    }

    /// Toggles vertical sync.
    pub fn set_vsync(&mut self, vsync: bool) -> bool {
        unsafe { SDL_RenderSetVSync(self.renderer.as_ptr(), i32::from(vsync)) == 0 }
    }

    fn set_logical_size(&mut self, width: u32, height: u32) {
        let _ = unsafe {
            SDL_RenderSetLogicalSize(self.renderer.as_ptr(), width as i32, height as i32)
        };
        unsafe { SDL_SetWindowMinimumSize(self.window.as_ptr(), width as i32, height as i32) };
    }

    fn set_integer_scaling(&mut self, enable: bool) {
        let enable = unsafe { std::mem::transmute::<i32, SDL_bool>(i32::from(enable)) };
        let _ = unsafe { SDL_RenderSetIntegerScale(self.renderer.as_ptr(), enable) == 0 };
    }

    /// Sets the viewport for this canvas, changing how coordinates are used.
    pub fn set_viewport(&mut self, viewport: &Viewport) {
        self.set_logical_size(viewport.logical_size.0, viewport.logical_size.1);
        self.set_integer_scaling(matches!(viewport.scaling, ViewportScaling::Integer));
    }

    pub(crate) fn show_window(&self) {
        unsafe { SDL_ShowWindow(self.window.as_ptr()) };
    }

    /// Clears the screen.
    pub fn clear(&mut self, color: super::Color) {
        let (r, g, b, a) = color.to_tuple();
        let renderer = self.renderer.as_ptr();
        let _ = unsafe { SDL_SetRenderDrawColor(renderer, r, g, b, a) };
        let _ = unsafe { SDL_RenderClear(renderer) };
    }

    /// Displays the current frame.
    pub fn display(&mut self) {
        unsafe { SDL_RenderPresent(self.renderer.as_ptr()) };
    }

    /// Draws an object
    pub fn draw<T: Drawable>(&mut self, object: &T, transform: impl Into<Transform>) {
        object.draw(self, transform.into());
    }

    /// Draws vertices on the screen.
    pub fn draw_geometry(
        &mut self,
        texture: &Texture,
        vertices: &[Vertex],
        indices: Option<&[i32]>,
    ) {
        unsafe {
            SDL_RenderGeometry(
                self.renderer.as_ptr(),
                texture.raw(),
                // Vertex and SDL_Vertex have the same layout, as Vec2 is also repr(C)
                vertices.as_ptr().cast::<sdl2_sys::SDL_Vertex>(),
                vertices.len() as i32,
                indices.map_or(std::ptr::null(), <[_]>::as_ptr),
                indices.map_or(0, <[_]>::len) as i32,
            )
        };
    }
}

/// Some information about the canvas' output
#[derive(Default)]
pub struct DisplayMode {
    /// Window width.
    pub width: u32,
    /// Window height.
    pub height: u32,
    /// Refresh rate.
    pub refresh: u32,
    /// Name of the renderer being used.
    pub renderer: &'static str,
}
