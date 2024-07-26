use std::ffi::CStr;

use thiserror::Error;

use crate::gfx;

/// Internal SDL error. This usually means something in backend went wrong.
#[derive(Debug, Error)]
#[error("SDL error: {0}")]
pub struct SdlError(String);

impl SdlError {
    pub(crate) fn from_sdl() -> Self {
        let msg = unsafe { CStr::from_ptr(sdl2_sys::SDL_GetError()) };
        let msg = msg.to_str().expect("Invalid SDL_GetError. Run").to_owned();

        Self(msg)
    }
}

/// Common errors from the library.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to initialise a window and renderer for the canvas. This system might not be
    /// supported.
    #[error("Failed to initialise canvas: {0}")]
    Canvas(#[from] gfx::CanvasError),
    /// Failed to load a texture. It could be missing, corrupted, or have an unsupported format.
    #[error("Failed to load texture: {0}")]
    TextureLoad(#[from] gfx::TextureLoadError),
}
