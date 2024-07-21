use std::path::Path;
use std::rc::Rc;

use glam::vec2;
use image::io::Reader;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use crate::math::Rect;

use super::{draw_vertices, with_canvas, Canvas, Drawable, Transform, Vertex, CANVAS};

pub struct TextureData {
    ptr: *mut sdl2_sys::SDL_Texture,
    w: u32,
    h: u32,
}

impl TextureData {
    fn from_image(img: image::DynamicImage) -> Self {
        let w = img.width();
        let h = img.height();
        let (format, mut data) = if img.color().has_alpha() {
            (PixelFormatEnum::RGBA32, img.into_rgba8().into_raw())
        } else {
            (PixelFormatEnum::RGB24, img.into_rgb8().into_raw())
        };
        let pitch = w * format.byte_size_per_pixel() as u32;

        with_canvas(|canvas| {
            let surface = Surface::from_data(&mut data, w, h, pitch, format).unwrap();
            let ptr = canvas.create_texture_from_surface(surface).unwrap().raw();
            Self { ptr, w, h }
        })
    }

    pub fn raw(&self) -> *mut sdl2_sys::SDL_Texture {
        self.ptr
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        let _ = CANVAS.try_with(|canvas| {
            if canvas.borrow().is_some() {
                unsafe { sdl2_sys::SDL_DestroyTexture(self.ptr) }
            }
        });
    }
}

#[derive(Clone)]
pub struct Texture {
    data: Rc<TextureData>,
}

impl Texture {
    pub fn load(path: impl AsRef<Path>) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        Self::from_image(image)
    }

    pub fn from_image(img: image::DynamicImage) -> Self {
        let data = TextureData::from_image(img).into();
        Self { data }
    }

    pub fn slice(&self, rect: Rect) -> TextureSlice {
        let data = self.data.clone();
        TextureSlice { data, rect }
    }

    pub fn width(&self) -> u32 {
        self.data.w
    }

    pub fn height(&self) -> u32 {
        self.data.h
    }
}

impl Drawable for &Texture {
    fn draw(&self, canvas: &mut Canvas, transform: Transform) {
        let points = [vec2(0., 0.), vec2(1., 0.), vec2(0., 1.), vec2(1., 1.)];
        let size = vec2(self.data.w as f32, self.data.h as f32);

        let verts = points.map(|p| Vertex::from_xy_uv(transform.transform_point(p * size), p));
        let idx = [0, 1, 2, 2, 1, 3];

        draw_vertices(canvas, &self.data, &verts, Some(&idx));
    }
}

#[derive(Clone)]
pub struct TextureSlice {
    data: Rc<TextureData>,
    rect: Rect,
}

impl Drawable for &TextureSlice {
    fn draw(&self, canvas: &mut Canvas, transform: Transform) {
        let points = [vec2(0., 0.), vec2(1., 0.), vec2(0., 1.), vec2(1., 1.)];
        let size = vec2(self.rect.w as f32, self.rect.h as f32);
        let uv = vec2(
            self.rect.x as f32 / self.data.w as f32,
            self.rect.y as f32 / self.data.h as f32,
        );
        let uv_size = vec2(
            self.rect.w as f32 / self.data.w as f32,
            self.rect.h as f32 / self.data.h as f32,
        );

        let verts = points
            .map(|p| Vertex::from_xy_uv(transform.transform_point(p * size), p * uv_size + uv));
        let idx = [0, 1, 2, 2, 1, 3];

        draw_vertices(canvas, &self.data, &verts, Some(&idx));
    }
}
