use std::f32::consts::{PI, TAU};
use umath::FF32;

const FRACT_PI_2: f32 = PI / 2.0;

/// faster cos
#[inline]
pub fn cos(x: f32) -> f32 {
    let x = unsafe { FF32::new(x) };
    let mut y = x * unsafe { FF32::new(1.0 / TAU) };
    y -= 0.25 + (y + 0.25).floor();
    y *= 16.0 * (y.abs() - 0.5);
    return *y;
}

/// faster sin
#[inline]
pub fn sin(x: f32) -> f32 {
    cos(x - FRACT_PI_2)
}

/// Calculates `a * b + c`, with hardware support if possible.
#[allow(clippy::suboptimal_flops)]
#[inline]
pub fn madd(a: f32, b: f32, c: f32) -> f32 {
    if cfg!(target_feature = "fma") {
        a.mul_add(b, c)
    } else {
        a * b + c
    }
}
