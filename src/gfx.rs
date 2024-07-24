//! Rendering and resource loading

use std::cell::RefCell;

pub use ecolor::Color32 as Color;
use glam::Vec2;

mod canvas;
mod texture;
mod transform;
pub use canvas::{Canvas, CanvasError, DisplayMode, Viewport, ViewportScaling};
pub use texture::{
    LoadError as TextureLoadError, Options as TextureOptions, Origin, ScaleMode, Texture,
    TextureSlice,
};
pub use transform::Transform;

thread_local! {
    pub(crate) static CANVAS: RefCell<Option<Canvas>> = const { RefCell::new(None) };
}

pub fn with_canvas<T>(f: impl FnOnce(&mut Canvas) -> T) -> T {
    CANVAS.with_borrow_mut(|canvas| f(canvas.as_mut().expect("no active renderer")))
}

#[repr(C)]
pub struct Vertex {
    pub coord: Vec2,
    pub color: Color,
    pub uv: Vec2,
}

impl Vertex {
    #[must_use]
    pub const fn from_xy_uv(coord: Vec2, uv: Vec2) -> Self {
        let color = Color::WHITE;
        Self { coord, color, uv }
    }
}

pub fn clear(color: Color) {
    with_canvas(|canvas| canvas.clear(color))
}

/// Display the current frame.
///
/// This is usually already called for you.
pub fn display() {
    with_canvas(Canvas::display)
}

/// Draw some [`Drawable`] object onto the screen.
///
/// This is the main drawing function. It can draw [textures][Texture] and [slices][TextureSlice],
/// positioned according to the [transform][Transform].
///
/// # Examples
///
/// ```
/// // Draw a texture at 40, 10
/// gfx::draw(texture, vec2(40., 10.));
/// // Draw it with 5x scale
/// gfx::draw(texture, (vec2(40., 10.), vec2(5., 5.)))
/// // Draw it with 180deg rotation
/// gfx::draw(texture, (vec2(40., 10.), vec2(5., 5.), PI))
/// ```
pub fn draw<T: Drawable>(object: &T, transform: impl Into<Transform>) {
    with_canvas(|canvas| object.draw(canvas, transform.into()))
}

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, transform: Transform);
}
