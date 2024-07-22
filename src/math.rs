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

#[must_use]
pub fn degrees(rad: f32) -> f32 {
    rad.to_radians()
}
