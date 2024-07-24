use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use sdl2::VideoSubsystem;
use sdl2_sys::{
    SDL_CreateRenderer, SDL_CreateWindow, SDL_EventType, SDL_GetRendererInfo,
    SDL_GetWindowDisplayMode, SDL_GetWindowSize, SDL_PollEvent, SDL_RenderClear,
    SDL_RenderGeometry, SDL_RenderPresent, SDL_RenderSetVSync, SDL_Renderer,
    SDL_SetRenderDrawColor, SDL_SetWindowSize, SDL_SetWindowTitle, SDL_ShowWindow, SDL_Window,
    SDL_WINDOWPOS_UNDEFINED_MASK,
};
use thiserror::Error;

use crate::{input, Drawable, SdlError, Texture, Transform, Vertex};

#[derive(Debug, Error)]
pub enum CanvasError {
    #[error(transparent)]
    Sdl(#[from] SdlError),
}

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
        unsafe {
            loop {
                let mut event = MaybeUninit::uninit();
                if SDL_PollEvent(event.as_mut_ptr()) == 0 {
                    return true;
                }
                let event = event.assume_init();
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
    }

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

    pub fn set_window_title(&mut self, title: &str) {
        unsafe { SDL_SetWindowTitle(self.window.as_ptr(), title.as_ptr().cast()) };
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        unsafe { SDL_SetWindowSize(self.window.as_ptr(), width as i32, height as i32) };
    }

    pub fn set_vsync(&mut self, vsync: bool) -> bool {
        let ok = unsafe { SDL_RenderSetVSync(self.renderer.as_ptr(), i32::from(vsync)) } == 0;
        if ok {
            log::warn!("Failed to enable vsync!");
        }
        ok
    }

    pub fn show_window(&self) {
        unsafe { SDL_ShowWindow(self.window.as_ptr()) };
    }

    pub fn clear(&mut self, color: super::Color) {
        let (r, g, b, a) = color.to_tuple();
        let renderer = self.renderer.as_ptr();
        let _ = unsafe { SDL_SetRenderDrawColor(renderer, r, g, b, a) };
        let _ = unsafe { SDL_RenderClear(renderer) };
    }

    pub fn display(&mut self) {
        unsafe { SDL_RenderPresent(self.renderer.as_ptr()) };
    }

    pub fn draw<T: Drawable>(&mut self, object: &T, transform: impl Into<Transform>) {
        object.draw(self, transform.into());
    }

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

#[derive(Default)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
    pub renderer: &'static str,
}
