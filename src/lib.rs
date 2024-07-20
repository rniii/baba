use thiserror::Error;

pub mod game;
pub mod gfx;
pub use game::{game, Game};
pub use prelude::*;

#[derive(Debug, Error)]
#[error("SDL error: {0}")]
pub struct SdlError(pub(crate) String);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sdl(#[from] SdlError),
    #[error("Invalid string: {0}")]
    InvalidString(#[from] std::ffi::NulError),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn run<State: Default>(title: impl AsRef<str>, update: impl Fn(&mut State)) -> Result {
    game(title, update).run()
}

pub mod prelude {
    pub use crate::gfx;
    pub use gfx::{Color, Texture, Transform, Vertex};

    pub use glam::{
        self, Affine2, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3,
        UVec4, Vec2, Vec3, Vec4,
    };
    pub use image;
}
