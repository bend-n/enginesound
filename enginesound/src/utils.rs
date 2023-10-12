use std::f32::consts::{PI, TAU};
const FRACT_PI_2: f32 = PI / 2.0;

/// faster cos
#[inline]
pub fn cos(x: f32) -> f32 {
    let mut y = x * (1.0 / TAU);
    if y.is_nan() {
        unsafe { std::hint::unreachable_unchecked() }
    }
    y -= 0.25 + (y + 0.25).floor();
    y *= 16.0 * (y.abs() - 0.5);
    return y;
}

/// faster sin
#[inline]
pub fn sin(x: f32) -> f32 {
    cos(x - FRACT_PI_2)
}
