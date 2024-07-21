use thiserror::Error;

pub mod game;
pub mod gfx;
pub mod input;
pub mod math;
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

pub fn run<State: Default>(title: impl Into<String>, update: impl Fn(&mut State)) -> Result {
    game(title, update).run()
}

pub mod prelude {
    pub use crate::gfx::{self, Color, Texture, TextureSlice, Transform, Vertex};
    pub use crate::input::{self, is_key_down, is_key_pressed, KeyCode};
    pub use crate::math::{
        dvec2, dvec3, dvec4, ivec2, ivec3, ivec4, mat2, mat3, mat4, uvec2, uvec3, uvec4, vec2,
        vec3, vec4, Affine2, DVec2, DVec3, DVec4, EulerRot, FloatExt, IVec2, IVec3, IVec4, Mat2,
        Mat3, Mat4, Rect, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
    };

    pub use glam;
    pub use image;
}
