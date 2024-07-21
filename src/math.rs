pub use glam::*;
pub use std::f32::consts::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    #[must_use]
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

pub struct Degrees(pub f32);

impl From<f32> for Degrees {
    fn from(value: f32) -> Self {
        Self(value.to_degrees())
    }
}

impl From<Degrees> for f32 {
    fn from(value: Degrees) -> Self {
        value.0.to_radians()
    }
}
