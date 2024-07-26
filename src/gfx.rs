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

/// Provides access to the currently active [`Canvas`]
pub fn with_canvas<T>(f: impl FnOnce(&mut Canvas) -> T) -> T {
    CANVAS.with_borrow_mut(|canvas| f(canvas.as_mut().expect("no active renderer")))
}

/// A point on the screen with texture coordinates and color.
///
/// This is a rendering primitive.
#[repr(C)]
pub struct Vertex {
    /// 2D position of the vertex on screen.
    pub coord: Vec2,
    /// Color of the vertex.
    pub color: Color,
    /// Texture coordinate of the vertex. This should be [0, 1].
    pub uv: Vec2,
}

impl Vertex {
    /// Create a vertex from 2d coordinates and uv coordinates.
    ///
    /// The color will be white.
    #[must_use]
    pub const fn from_xy_uv(coord: Vec2, uv: Vec2) -> Self {
        let color = Color::WHITE;
        Self { coord, color, uv }
    }
}

/// Clears the screen.
pub fn clear(color: Color) {
    with_canvas(|canvas| canvas.clear(color))
}

/// Display the current frame.
///
/// This is usually already called for you.
pub fn display() {
    with_canvas(Canvas::display)
}

/// Draws some [`Drawable`] object onto the screen.
///
/// This is the main drawing function. It can draw [textures][Texture] and [slices][TextureSlice],
/// positioned according to the [transform][Transform].
///
/// # Examples
///
/// ```no_run
/// # use baba::prelude::*;
/// # let texture = Texture::empty();
/// // Draw a texture at 40, 10
/// gfx::draw(&texture, vec2(40., 10.));
/// // Draw it with 5x scale
/// gfx::draw(&texture, (vec2(40., 10.), vec2(5., 5.)));
/// // Draw it with 180deg rotation
/// gfx::draw(&texture, (vec2(40., 10.), vec2(5., 5.), degrees(180.)));
/// ```
pub fn draw<T: Drawable>(object: &T, transform: impl Into<Transform>) {
    with_canvas(|canvas| object.draw(canvas, transform.into()))
}

/// Objects which can be drawn by [`draw`].
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a `Drawable` type",
    label = "Cannot be drawn"
)]
pub trait Drawable {
    /// Draws this object, applying a transform to it.
    ///
    /// You can use any of the drawing functions on the [`Canvas`]. Drawing your own geometry is
    /// possible via [`draw_geometry`][Canvas::draw_geometry].
    fn draw(&self, canvas: &mut Canvas, transform: Transform);
}
