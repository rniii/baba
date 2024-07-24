use std::ffi::CStr;

use thiserror::Error;

use crate::gfx;

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to initialise canvas")]
    Canvas(#[from] gfx::CanvasError),
    #[error("Failed to load texture")]
    TextureLoad(#[from] gfx::TextureLoadError),
}
