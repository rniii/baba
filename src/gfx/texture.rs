use std::path::Path;
use std::rc::Rc;

use glam::{vec2, Vec2};
use image::io::Reader;
use sdl2::pixels::PixelFormatEnum;
use thiserror::Error;

use crate::math::Rect;
use crate::SdlError;

use super::{with_canvas, Canvas, Drawable, Transform, Vertex};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Decode(#[from] image::ImageError),
    #[error(transparent)]
    Sdl(#[from] SdlError),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Origin(pub Vec2);

impl Origin {
    pub const TOP_LEFT: Self = Self(Vec2::ZERO);
    pub const TOP_RIGHT: Self = Self(Vec2::X);
    pub const BOTTOM_LEFT: Self = Self(Vec2::Y);
    pub const BOTTOM_RIGHT: Self = Self(Vec2::ONE);
    pub const CENTER: Self = Self(Vec2::splat(0.5));
}

#[derive(Default, Clone, Copy)]
#[repr(u32)]
pub enum ScaleMode {
    #[default]
    Nearest = 0,
    Linear = 1,
    // No SDL2 backend uses this, seemingly
    // Anisotropic = 2,
}

#[derive(Default)]
pub struct Options {
    // blend: BlendMode,
    pub scaling: Option<ScaleMode>,
    pub origin: Origin,
}

impl From<ScaleMode> for Options {
    fn from(scaling: ScaleMode) -> Self {
        Self {
            scaling: Some(scaling),
            ..Default::default()
        }
    }
}

impl From<Origin> for Options {
    fn from(origin: Origin) -> Self {
        Self {
            origin,
            ..Default::default()
        }
    }
}

pub struct TextureData {
    ptr: *mut sdl2_sys::SDL_Texture,
    w: u32,
    h: u32,
}

impl TextureData {
    const fn empty() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            w: 0,
            h: 0,
        }
    }

    fn from_image(img: image::DynamicImage, opts: &Options) -> Result<Self, LoadError> {
        let w = img.width();
        let h = img.height();
        let (format, mut data) = if img.color().has_alpha() {
            (PixelFormatEnum::RGBA32, img.into_rgba8().into_raw())
        } else {
            (PixelFormatEnum::RGB24, img.into_rgb8().into_raw())
        };
        let pitch = w * format.byte_size_per_pixel() as u32;

        with_canvas(|canvas| unsafe {
            let surface = sdl2_sys::SDL_CreateRGBSurfaceWithFormatFrom(
                data.as_mut_ptr().cast(),
                w as i32,
                h as i32,
                /* unused */ 0,
                pitch as i32,
                format as u32,
            );
            if surface.is_null() {
                return Err(SdlError::from_sdl())?;
            }

            let ptr = sdl2_sys::SDL_CreateTextureFromSurface(canvas.renderer(), surface);
            if ptr.is_null() {
                log::warn!("Failed to create a texture: {}", SdlError::from_sdl());
            }

            if let Some(scale) = opts.scaling {
                let scale = std::mem::transmute::<ScaleMode, sdl2_sys::SDL_ScaleMode>(scale);
                sdl2_sys::SDL_SetTextureScaleMode(ptr, scale);
            }

            Ok(Self { ptr, w, h })
        })
    }

    pub const fn raw(&self) -> *mut sdl2_sys::SDL_Texture {
        self.ptr
    }
}

impl Drop for TextureData {
    fn drop(&mut self) {
        unsafe { sdl2_sys::SDL_DestroyTexture(self.ptr) }
    }
}

#[must_use]
#[derive(Clone)]
pub struct Texture {
    data: Rc<TextureData>,
    origin: Vec2,
}

impl Texture {
    pub fn empty() -> Self {
        let data = Rc::new(TextureData::empty());
        let origin = Vec2::ZERO;
        Self { data, origin }
    }

    pub fn load(path: impl AsRef<Path>) -> Self {
        Self::load_with(path, Options::default())
    }

    pub fn load_with(path: impl AsRef<Path>, options: impl Into<Options>) -> Self {
        Self::try_load(path.as_ref(), options)
            .inspect_err(|e| log::error!("Failed to load {}: {e}", path.as_ref().display()))
            .unwrap_or_else(|_| Self::empty())
    }

    pub fn try_load(
        path: impl AsRef<Path>,
        options: impl Into<Options>,
    ) -> Result<Self, LoadError> {
        Self::from_image(Reader::open(path.as_ref())?.decode()?, options.into())
    }

    pub fn from_image(
        img: image::DynamicImage,
        options: impl Into<Options>,
    ) -> Result<Self, LoadError> {
        let options = options.into();
        let origin = options.origin.0;
        let data = Rc::new(TextureData::from_image(img, &options)?);
        Ok(Self { data, origin })
    }

    pub fn slice(&self, rect: Rect) -> TextureSlice {
        let texture = self.clone();
        TextureSlice { texture, rect }
    }

    pub const fn with_origin(mut self, origin: Origin) -> Self {
        self.origin = origin.0;
        self
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.data.w
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.data.h
    }

    pub(crate) fn raw(&self) -> *mut sdl2_sys::SDL_Texture {
        self.data.raw()
    }
}

#[must_use]
#[derive(Clone)]
pub struct TextureSlice {
    texture: Texture,
    rect: Rect,
}

const QUAD_VERTS: [Vec2; 4] = [vec2(0., 0.), vec2(1., 0.), vec2(0., 1.), vec2(1., 1.)];
const QUAD_IDX: [i32; 6] = [0, 1, 2, 2, 1, 3];

impl Drawable for Texture {
    fn draw(&self, canvas: &mut Canvas, transform: Transform) {
        let size = vec2(self.data.w as f32, self.data.h as f32);
        let transform = transform.scale(size);
        let verts =
            QUAD_VERTS.map(|p| Vertex::from_xy_uv(transform.transform_point(p - self.origin), p));

        canvas.draw_geometry(self, &verts, Some(&QUAD_IDX));
    }
}

impl Drawable for TextureSlice {
    fn draw(&self, canvas: &mut Canvas, transform: Transform) {
        let data = &self.texture.data;
        let origin = self.texture.origin;

        let size = vec2(self.rect.w as f32, self.rect.h as f32);
        let uv = vec2(
            self.rect.x as f32 / data.w as f32,
            self.rect.y as f32 / data.h as f32,
        );
        let uv_size = vec2(
            self.rect.w as f32 / data.w as f32,
            self.rect.h as f32 / data.h as f32,
        );
        let transform = transform.scale(size);

        let verts = QUAD_VERTS
            .map(|p| Vertex::from_xy_uv(transform.transform_point(p - origin), p * uv_size + uv));

        canvas.draw_geometry(&self.texture, &verts, Some(&QUAD_IDX));
    }
}
