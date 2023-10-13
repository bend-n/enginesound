use std::f32::consts::{PI, TAU};
use umath::FF32;

const FRACT_PI_2: f32 = PI / 2.0;

pub trait FExt {
    /// Calculates `a * b + c`, with hardware support if possible.
    fn madd(self, a: f32, b: f32) -> Self;
    /// faster cos
    fn cosf(self) -> Self;
    /// faster sin
    fn sinf(self) -> Self;
}

impl FExt for f32 {
    #[allow(clippy::suboptimal_flops)]
    #[inline]
    fn madd(self, b: f32, c: f32) -> f32 {
        if cfg!(target_feature = "fma") {
            self.mul_add(b, c)
        } else {
            self * b + c
        }
    }

    #[inline]
    fn cosf(self) -> f32 {
        let x = unsafe { FF32::new(self) };
        let mut y = x * unsafe { FF32::new(1.0 / TAU) };
        y -= 0.25 + (y + 0.25).floor();
        *(y * 16.0 * (y.abs() - 0.5))
    }

    #[inline]
    fn sinf(self) -> f32 {
        (self - FRACT_PI_2).cosf()
    }
}
