use std::path::Path;

use glam::vec2;
use image::io::Reader;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render;
use sdl2::surface::Surface;

use super::{draw_vertices, Canvas, Drawable, Transform, Vertex, CANVAS};

pub struct Texture {
    inner: render::Texture,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn load(path: impl AsRef<Path>) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        Self::from_image(image)
    }

    pub fn from_image(img: image::DynamicImage) -> Self {
        CANVAS.with_borrow(|canvas| {
            let canvas = canvas.as_ref().expect("no active renderer");
            let width = img.width();
            let height = img.height();
            let (format, mut data) = if img.color().has_alpha() {
                (PixelFormatEnum::RGBA8888, img.into_rgba8().into_raw())
            } else {
                (PixelFormatEnum::RGB888, img.into_rgb8().into_raw())
            };
            let pitch = width * format.byte_size_per_pixel() as u32;
            let surface = Surface::from_data(&mut data, width, height, pitch, format).unwrap();
            let inner = canvas.create_texture_from_surface(surface).unwrap();

            Texture {
                inner,
                width,
                height,
            }
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn raw(&self) -> *mut sdl2_sys::SDL_Texture {
        self.inner.raw()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let _ = CANVAS.try_with(|canvas| {
            if canvas.borrow().is_some() {
                unsafe { sdl2_sys::SDL_DestroyTexture(self.inner.raw()) }
            }
        });
    }
}

impl Drawable for &Texture {
    fn draw(&self, canvas: &mut Canvas, transform: Transform) {
        let points = [vec2(0., 0.), vec2(1., 0.), vec2(0., 1.), vec2(1., 1.)];
        let size = vec2(self.width as f32, self.height as f32);

        let verts = points.map(|p| Vertex::from_xy_uv(transform.transform_point(p * size), p));
        let idx = [0, 1, 2, 2, 1, 3];

        draw_vertices(canvas, self, &verts, Some(&idx));
    }
}
