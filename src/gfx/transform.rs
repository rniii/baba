use std::ops::Mul;

use crate::math::{Affine2, Degrees, Mat2, Mat3, Vec2};

#[must_use]
#[derive(Debug, Default)]
pub struct Transform(Affine2);

/// Two-dimensional coordinate transformation.
impl Transform {
    pub const IDENTITY: Self = Self::from_affine(Affine2::IDENTITY);

    /// Create a transform from an affine transformation matrix.
    #[inline(always)]
    pub const fn from_affine(aff: Affine2) -> Self {
        Self(aff)
    }

    /// Create a transform with translation.
    #[inline(always)]
    pub fn from_translation(coords: Vec2) -> Self {
        Self(Affine2::from_translation(coords))
    }

    /// Create a transform with scale.
    #[inline(always)]
    pub fn from_scale(scale: Vec2) -> Self {
        Self(Affine2::from_scale(scale))
    }

    /// Create a transform with `angle` (in radians).
    #[inline(always)]
    pub fn from_rotation(angle: f32) -> Self {
        Self(Affine2::from_angle(angle))
    }

    /// Translate this transform by `coords`.
    #[inline(always)]
    pub fn translate(self, coords: Vec2) -> Self {
        self * Self::from_translation(coords)
    }

    /// Scale this transform by `scale`.
    #[inline(always)]
    pub fn scale(self, scale: Vec2) -> Self {
        self * Self::from_scale(scale)
    }

    /// Rotate this transform by `angle` (in radians).
    #[inline(always)]
    pub fn rotate(self, angle: f32) -> Self {
        self * Self::from_rotation(angle)
    }

    #[must_use]
    pub const fn to_affine(self) -> Affine2 {
        self.0
    }

    /// Transform a 2D point with this object.
    ///
    /// This may translate, scale and rotate.
    #[must_use]
    #[inline(always)]
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        self.0.transform_point2(point)
    }
}

impl Mul for Transform {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl From<Mat3> for Transform {
    #[inline(always)]
    fn from(value: Mat3) -> Self {
        Self(Affine2::from_mat3(value))
    }
}

impl From<Mat2> for Transform {
    #[inline(always)]
    fn from(value: Mat2) -> Self {
        Self(Affine2::from_mat2(value))
    }
}

macro_rules! impl_translation_scale_rotation {
    ($T:ty, $U:ty, $V:ty) => {
        impl From<($T, $U, $V)> for Transform {
            /// Create a transform with translation, scale and rotation.
            #[inline(always)]
            fn from((translation, scale, angle): ($T, $U, $V)) -> Self {
                Self::from_translation(translation.into())
                    .scale(scale.into())
                    .rotate(angle.into())
            }
        }
    };
}

macro_rules! impl_translation_scale {
    ($T:ty, $U:ty) => {
        impl From<($T, $U)> for Transform {
            /// Create a transform with translation and scale.
            #[inline(always)]
            fn from((translation, scale): ($T, $U)) -> Self {
                Self::from_translation(translation.into()).scale(scale.into())
            }
        }

        impl_translation_scale_rotation!($T, $U, f32);
        impl_translation_scale_rotation!($T, $U, Degrees);
    };
}

macro_rules! impl_translation {
    ($T:ty) => {
        impl From<$T> for Transform {
            /// Create a transform with translation.
            #[inline(always)]
            fn from(value: $T) -> Self {
                Self::from_translation(value.into())
            }
        }

        impl_translation_scale!($T, Vec2);
        impl_translation_scale!($T, (f32, f32));
        impl_translation_scale!($T, [f32; 2]);
    };
}

impl_translation!(Vec2);
impl_translation!((f32, f32));
impl_translation!([f32; 2]);
