#![warn(
    clippy::pedantic,
    clippy::missing_const_for_fn,
    clippy::use_self,
    unsafe_op_in_unsafe_fn
)]
#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

mod error;
mod game;
pub mod gfx;
pub mod input;
pub mod math;
pub use error::{Error, SdlError};
pub use game::Game;
#[doc(inline)]
pub use prelude::*;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub fn game<S>(name: impl Into<String>, update: impl Fn(&mut S)) -> Game<S, impl Fn(&mut S)> {
    Game::new(name.into(), update)
}

pub fn run<S>(name: impl Into<String>, update: impl Fn(&mut S)) -> Result
where
    S: Default,
{
    game(name, update).run()
}

pub mod prelude {
    pub use crate::game::{Framerate, Settings, WindowSettings};
    pub use crate::gfx::{
        self, Color, Drawable, Origin, ScaleMode, Texture, TextureOptions, TextureSlice, Transform,
        Vertex, Viewport, ViewportScaling,
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
