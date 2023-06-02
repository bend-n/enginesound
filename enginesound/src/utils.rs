use crate::gen::{Engine, LoopBuffer, LowPassFilter};

pub const SPEED_OF_SOUND: f32 = 343.0; // m/s

/// faster cos
#[inline]
pub fn cos(x: f32) -> f32 {
    let mut y = x * (1.0 / 6.283); // TAU
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
    cos(x - 1.570) // FRACT_PI_2
}

/// converts a given amount of time into samples
#[inline]
pub fn seconds_to_samples(seconds: f32, sample_rate: u32) -> usize {
    (seconds * sample_rate as f32).max(1.0) as usize
}

/// converts a given distance into samples via the speed of sound
#[inline]
pub fn distance_to_samples(meters: f32, sample_rate: u32) -> usize {
    seconds_to_samples(meters / SPEED_OF_SOUND, sample_rate)
}

#[inline]
pub fn samples_to_seconds(samples: usize, sample_rate: u32) -> f32 {
    samples as f32 / sample_rate as f32
}

/// returns meters
#[inline]
pub fn samples_to_distance(samples: usize, sample_rate: u32) -> f32 {
    samples_to_seconds(samples, sample_rate) * SPEED_OF_SOUND
}

/// Deserialization is not fully implemented via serde because we need the sample rate to set up delay buffers
pub fn fix_engine(engine: &mut Engine, sample_rate: u32) {
    fn fix_lpf(lpf: &mut LowPassFilter, sample_rate: u32) {
        *lpf = LowPassFilter::new(1.0 / lpf.delay, sample_rate);
    }

    fn fix_loop_buffer(lb: &mut LoopBuffer, sample_rate: u32) {
        let len = (lb.delay * sample_rate as f32) as usize;

        *lb = LoopBuffer {
            delay: lb.delay,
            data: vec![0.0; len],
            pos: 0,
        };
    }

    vec![
        &mut engine.crankshaft_fluctuation_lp,
        &mut engine.engine_vibration_filter,
        &mut engine.intake_noise_lp,
    ]
    .into_iter()
    .for_each(|lpf| fix_lpf(lpf, sample_rate));

    engine
        .muffler
        .muffler_elements
        .iter_mut()
        .chain(std::iter::once(&mut engine.muffler.straight_pipe))
        .flat_map(|waveguide| vec![&mut waveguide.chamber0, &mut waveguide.chamber1].into_iter())
        .chain(engine.cylinders.iter_mut().flat_map(|cylinder| {
            vec![
                &mut cylinder.exhaust_waveguide.chamber0,
                &mut cylinder.exhaust_waveguide.chamber1,
                &mut cylinder.extractor_waveguide.chamber0,
                &mut cylinder.extractor_waveguide.chamber1,
                &mut cylinder.intake_waveguide.chamber0,
                &mut cylinder.intake_waveguide.chamber1,
            ]
            .into_iter()
        }))
        .for_each(|delay_line| fix_loop_buffer(&mut delay_line.samples, sample_rate));
}
