use std::cell::RefCell;

pub use ecolor::Color32 as Color;
use glam::Vec2;
pub use sdl2::render::WindowCanvas as Canvas;

mod texture;
mod transform;
pub use texture::{Texture, TextureSlice};
pub use transform::Transform;

thread_local! {
    pub(crate) static CANVAS: RefCell<Option<Canvas>> = const { RefCell::new(None) };
}

fn with_canvas<T>(f: impl FnOnce(&mut Canvas) -> T) -> T {
    CANVAS.with_borrow_mut(|canvas| f(canvas.as_mut().expect("no active renderer")))
}

#[repr(C)]
pub struct Vertex {
    pub coord: Vec2,
    pub color: Color,
    pub uv: Vec2,
}

impl Vertex {
    fn from_xy_uv(coord: Vec2, uv: Vec2) -> Self {
        let color = Color::WHITE;
        Self { coord, uv, color }
    }
}

pub fn clear(color: Color) {
    with_canvas(|canvas| {
        canvas.set_draw_color(color.to_tuple());
        canvas.clear();
    })
}

pub fn display() {
    with_canvas(|canvas| canvas.present())
}

pub fn draw<T: Drawable>(object: T, transform: impl Into<Transform>) {
    with_canvas(|canvas| object.draw(canvas, transform.into()))
}

pub(crate) fn draw_vertices(
    canvas: &mut Canvas,
    texture: &texture::TextureData,
    vertices: &[Vertex],
    indices: Option<&[i32]>,
) {
    unsafe {
        sdl2_sys::SDL_RenderGeometry(
            canvas.raw(),
            texture.raw(),
            // Vertex and SDL_Vertex have the same layout, as Vec2 is also repr(C)
            vertices.as_ptr().cast::<sdl2_sys::SDL_Vertex>(),
            vertices.len() as i32,
            indices.map_or(std::ptr::null(), |i| i.as_ptr()),
            indices.map_or(0, |i| i.len()) as i32,
        )
    };
}

pub trait Drawable: private::Sealed {
    #[doc(hidden)]
    fn draw(&self, canvas: &mut Canvas, transform: Transform);
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for &Texture {}
    impl Sealed for &TextureSlice {}
}
