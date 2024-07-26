//! Common math operations.
//!
//! This supports linear algebra using [`glam`].

pub use glam::{
    dvec2, dvec3, dvec4, ivec2, ivec3, ivec4, mat2, mat3, mat4, uvec2, uvec3, uvec4, vec2, vec3,
    vec4, Affine2, DVec2, DVec3, DVec4, EulerRot, FloatExt, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4,
    UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
pub use std::f32::consts::{E, LN_10, LN_2, LOG10_2, LOG10_E, LOG2_10, LOG2_E, PI, SQRT_2, TAU};

/// Defines a rectangle bounding.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rect {
    /// Horizontal position
    pub x: u32,
    /// Vertical position
    pub y: u32,
    /// Horizontal size
    pub w: u32,
    /// Vertical size
    pub h: u32,
}

impl Rect {
    /// Create a new rectangle with position and size.
    #[must_use]
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

/// Converts degrees to radians. All engine functions expect radians!
#[must_use]
pub fn degrees(rad: f32) -> f32 {
    rad.to_radians()
}
