#![warn(
    clippy::pedantic,
    clippy::missing_const_for_fn,
    clippy::use_self,
    unsafe_op_in_unsafe_fn,
)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::unnecessary_wraps,
    clippy::needless_pass_by_value,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports,
    clippy::missing_errors_doc
)]

use thiserror::Error;

mod game;
pub mod gfx;
pub mod input;
pub mod math;
pub use game::{game, run, Game, Settings, WindowSettings};
#[doc(inline)]
pub use prelude::*;

#[derive(Debug, Error)]
#[error("SDL error: {0}")]
pub struct SdlError(pub(crate) String);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Sdl(#[from] SdlError),
    #[error(transparent)]
    TextureLoad(#[from] gfx::TextureLoadError),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub mod prelude {
    pub use crate::gfx::{
        self, Color, Origin, ScaleMode, Texture, TextureOptions, TextureSlice, Transform, Vertex,
    };
    pub use crate::input::{self, is_key_down, is_key_pressed, KeyCode};
    pub use crate::math::{
        degrees, dvec2, dvec3, dvec4, ivec2, ivec3, ivec4, mat2, mat3, mat4, uvec2, uvec3, uvec4,
        vec2, vec3, vec4, Affine2, DVec2, DVec3, DVec4, EulerRot, FloatExt, IVec2, IVec3, IVec4,
        Mat2, Mat3, Mat4, Rect, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
    };
    pub use std::f32::consts::*;

    pub use glam;
    pub use image;
    pub use log::{debug, info, log, trace, warn, Level as LogLevel};
}
