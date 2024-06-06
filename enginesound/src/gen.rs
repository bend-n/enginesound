//! ## Generator module ##
//!
//! Basic working principle:
//! Every sample-output generating object (Cylinder, `WaveGuide`, `DelayLine`, ..) has to be first `pop`ped,
//! it's output worked upon and then new input samples are `push`ed.
//!
#![warn(clippy::suboptimal_flops, clippy::use_self, clippy::dbg_macro)]
#[cfg(feature = "godot")]
use godot::{engine::AudioStreamGeneratorPlayback, prelude::*};
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::time::SystemTime;

use crate::utils::FExt;

pub const PI2F: f32 = 2.0 * std::f32::consts::PI;
pub const PI4F: f32 = 4.0 * std::f32::consts::PI;
pub const WAVEGUIDE_MAX_AMP: f32 = 20.0; // at this amplitude, a damping function is applied to fight feedback loops

// https://www.researchgate.net/profile/Stefano_Delle_Monache/publication/280086598_Physically_informed_car_engine_sound_synthesis_for_virtual_and_augmented_environments/links/55a791bc08aea2222c746724/Physically-informed-car-engine-sound-synthesis-for-virtual-and-augmented-environments.pdf?origin=publication_detail

#[derive(Default)]
pub struct Muffler {
    pub straight_pipe: WaveGuide,
    pub muffler_elements: Box<[WaveGuide]>,
}

#[derive(Default)]
pub struct Engine {
    pub rpm: f32,
    pub intake_volume: f32,
    pub exhaust_volume: f32,
    pub engine_vibrations_volume: f32,

    pub cylinders: Box<[Cylinder]>,
    // #[serde(skip)]
    pub intake_noise: Noise,
    pub intake_noise_factor: f32,
    pub intake_noise_lp: LowPassFilter,
    pub engine_vibration_filter: LowPassFilter,
    pub muffler: Muffler,
    /// valve timing -0.5 - 0.5
    pub intake_valve_shift: f32,
    /// valve timing -0.5 - 0.5
    pub exhaust_valve_shift: f32,
    pub crankshaft_fluctuation: f32,
    pub crankshaft_fluctuation_lp: LowPassFilter,
    // #[serde(skip)]
    pub crankshaft_noise: Noise,
    // running values
    /// crankshaft position, 0.0-1.0
    // #[serde(skip)]
    pub crankshaft_pos: f32,
    // #[serde(skip)]
    pub exhaust_collector: f32,
    // #[serde(skip)]
    pub intake_collector: f32,
}

impl Engine {
    pub fn new(samples_per_second: u32) -> Self {
        macro_rules! wave {
            ($delay:literal, $alpha:literal,$beta:literal) => {
                WaveGuide::new(
                    ($delay * samples_per_second as f32) as usize,
                    $alpha,
                    $beta,
                    samples_per_second,
                )
            };
        }
        macro_rules! lpf {
            ($len:literal) => {
                LowPassFilter::new(1.0 / $len, samples_per_second)
            };
        }
        Self {
            rpm: 883.1155,
            intake_volume: 0.32493597,
            exhaust_volume: 0.63871837,
            engine_vibrations_volume: 0.036345694,
            cylinders: vec![
                Cylinder {
                    crank_offset: 0.0,
                    exhaust_waveguide: wave!(0.0009583333, 0.7145016, 0.06),
                    intake_waveguide: wave!(0.00014583333, 0.2054379, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.5,
                    exhaust_waveguide: wave!(0.0009583333, 0.7145016, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.666_666_7,
                    exhaust_waveguide: wave!(0.000_958_333_3, 0.472_959_76, 0.06),
                    intake_waveguide: wave!(0.000_145_833_33, 1.0, -0.757_582_7),
                    extractor_waveguide: wave!(0.000_583_333_3, 0.0, -0.000_812_947_75),
                    intake_open_refl: 0.006_074_19,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.000_701_546_67,
                    exhaust_closed_refl: 0.714_501_6,
                    piston_motion_factor: 2.559_478_3,
                    ignition_factor: 2.564_522_3,
                    ignition_time: 0.102_849_334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.75,
                    exhaust_waveguide: wave!(0.0009583333, 0.010738671, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.8,
                    exhaust_waveguide: wave!(0.0009583333, 0.070256054, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.8333333,
                    exhaust_waveguide: wave!(0.0009583333, 0.2522802, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.85714287,
                    exhaust_waveguide: wave!(0.0009583333, 0.43368497, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
                Cylinder {
                    crank_offset: 0.875,
                    exhaust_waveguide: wave!(0.0009583333, 0.587092, 0.06),
                    intake_waveguide: wave!(0.00014583333, 1.0, -0.7575827),
                    extractor_waveguide: wave!(0.0005833333, 0.0, -0.00081294775),
                    intake_open_refl: 0.00607419,
                    intake_closed_refl: 1.0,
                    exhaust_open_refl: -0.00070154667,
                    exhaust_closed_refl: 0.7145016,
                    piston_motion_factor: 2.5594783,
                    ignition_factor: 2.5645223,
                    ignition_time: 0.102849334,
                    ..Default::default()
                },
            ]
            .into(),
            intake_noise_factor: 1.3716942,
            intake_noise_lp: lpf!(0.0005277371),
            engine_vibration_filter: lpf!(0.010829452),
            muffler: Muffler {
                straight_pipe: wave!(0.0064375, 0.0063244104, 0.0016502142),
                muffler_elements: vec![
                    wave!(0.00014583333, 0.0, -0.14208126),
                    wave!(0.0001875, 0.0, -0.14208126),
                    wave!(0.00020833334, 0.0, -0.14208126),
                    wave!(0.00025, 0.0, -0.14208126),
                ]
                .into(),
            },
            intake_valve_shift: -0.041683555,
            exhaust_valve_shift: -0.0046506226,
            crankshaft_fluctuation: 0.4000154,
            crankshaft_fluctuation_lp: lpf!(0.086017124),
            ..Default::default()
        }
    }
}

pub struct Noise {
    inner: XorShiftRng,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            inner: XorShiftRng::from_seed(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
                    .to_le_bytes(),
            ),
        }
    }
}

impl Noise {
    pub fn step(&mut self) -> f32 {
        self.inner.next_u32() as f32 / (std::u32::MAX as f32 / 2.0) - 1.0
    }
}

/// Represents one audio cylinder
/// It has two `WaveGuide`s each connected from the cylinder to the exhaust or intake collector
/// ```
/// Labels:                                                     \/ Extractor
///                    b      a            a      b           a    b
/// (Intake Collector) <==|IV|> (Cylinder) <|EV|==> (Exhaust) <====> (Exhaust collector)
///
/// a   b
/// <===>   - WaveGuide with alpha / beta sides => alpha controls the reflectiveness of that side
///
/// |IV|    - Intake valve modulation function for this side of the WaveGuide (alpha)
///
/// |EV|    - Exhaust valve modulation function for this side of the WaveGuide (alpha)
/// ```
#[derive(Clone, Default)]
pub struct Cylinder {
    /// offset of this cylinder's piston crank
    pub crank_offset: f32,
    /// waveguide from the cylinder to the exhaust
    pub exhaust_waveguide: WaveGuide,
    /// waveguide from the cylinder to the intake
    pub intake_waveguide: WaveGuide,
    /// waveguide from the other end of the exhaust WG to the exhaust collector
    pub extractor_waveguide: WaveGuide,
    // waveguide alpha values for when the valves are closed or opened
    pub intake_open_refl: f32,
    pub intake_closed_refl: f32,
    pub exhaust_open_refl: f32,
    pub exhaust_closed_refl: f32,

    pub piston_motion_factor: f32,
    pub ignition_factor: f32,
    /// the time it takes for the fuel to ignite in crank cycles (0.0 - 1.0)
    pub ignition_time: f32,

    // running values
    // #[serde(skip)]
    pub cyl_sound: f32,
    // #[serde(skip)]
    pub extractor_exhaust: f32,
}

impl Cylinder {
    /// takes in the current exhaust collector pressure
    /// returns (intake, exhaust, piston + ignition, waveguide dampened)
    #[inline]
    fn pop(
        &mut self,
        crank_pos: f32,
        exhaust_collector: f32,
        intake_valve_shift: f32,
        exhaust_valve_shift: f32,
    ) -> (f32, f32, f32) {
        let crank = (crank_pos + self.crank_offset).fract();

        self.cyl_sound = piston_motion(crank).madd(
            self.piston_motion_factor,
            fuel_ignition(crank, self.ignition_time) * self.ignition_factor,
        );

        let ex_valve = exhaust_valve((crank + exhaust_valve_shift).fract());
        let in_valve = intake_valve((crank + intake_valve_shift).fract());

        self.exhaust_waveguide.alpha = (self.exhaust_open_refl - self.exhaust_closed_refl)
            .madd(ex_valve, self.exhaust_closed_refl);
        self.intake_waveguide.alpha = (self.intake_open_refl - self.intake_closed_refl)
            .madd(in_valve, self.intake_closed_refl);

        // the first return value in the tuple is the cylinder-side valve-modulated side of the waveguide (alpha side)
        let ex_wg_ret = self.exhaust_waveguide.pop();
        let in_wg_ret = self.intake_waveguide.pop();

        let extractor_wg_ret = self.extractor_waveguide.pop();
        self.extractor_exhaust = extractor_wg_ret.0;
        self.extractor_waveguide
            .push(ex_wg_ret.1, exhaust_collector);

        //self.cyl_sound += ex_wg_ret.0 + in_wg_ret.0;

        (in_wg_ret.1, extractor_wg_ret.1, self.cyl_sound)
    }

    /// called after pop
    fn push(&mut self, intake: f32) {
        let ex_in = (1.0 - self.exhaust_waveguide.alpha.abs()) * self.cyl_sound * 0.5;
        self.exhaust_waveguide.push(ex_in, self.extractor_exhaust);
        let in_in = (1.0 - self.intake_waveguide.alpha.abs()) * self.cyl_sound * 0.5;
        self.intake_waveguide.push(in_in, intake);
    }
}

pub struct Generator {
    pub volume: f32,
    pub samples_per_second: u32,
    pub engine: Engine,
    /// `LowPassFilter` which is subtracted from the sample while playing back to reduce dc offset and thus clipping
    dc_lp: LowPassFilter,
}

impl Generator {
    pub fn new(samples_per_second: u32, engine: Engine, dc_lp: LowPassFilter) -> Self {
        Self {
            volume: 0.1_f32,
            samples_per_second,
            engine,
            dc_lp,
        }
    }

    #[cfg(feature = "godot")]
    pub fn generate(&mut self, player: &mut Gd<AudioStreamGeneratorPlayback>) {
        let samples_per_second = self.samples_per_second as f32 * 120.0;

        let inc = self.engine.rpm / samples_per_second;

        for _ in 0..player.get_frames_available() {
            self.engine.crankshaft_pos = (self.engine.crankshaft_pos + inc).fract();

            let (intake, vibration, exhaust) = self.gen();
            let mixed = exhaust.madd(
                self.engine.exhaust_volume,
                intake.madd(
                    self.engine.intake_volume,
                    vibration * self.engine.engine_vibrations_volume,
                ),
            ) * self.volume;

            // reduces dc offset
            let sample = mixed - self.dc_lp.filter(mixed);
            player.push_frame(Vector2::splat(sample));
        }
    }

    pub fn reset(&mut self) {
        for cyl in self.engine.cylinders.iter_mut() {
            [
                &mut cyl.exhaust_waveguide,
                &mut cyl.intake_waveguide,
                &mut cyl.extractor_waveguide,
            ]
            .iter_mut()
            .flat_map(|x| [&mut x.chamber0, &mut x.chamber1])
            .for_each(|chamber| chamber.samples.data.iter_mut().for_each(|x| *x = 0.0));

            cyl.extractor_exhaust = 0.0;
            cyl.cyl_sound = 0.0;
        }

        std::iter::once(&mut self.engine.muffler.straight_pipe)
            .flat_map(|x| [&mut x.chamber0, &mut x.chamber1])
            .for_each(|chamber| chamber.samples.data.iter_mut().for_each(|x| *x = 0.0));

        for muffler_element in self.engine.muffler.muffler_elements.iter_mut() {
            muffler_element
                .chamber0
                .samples
                .data
                .iter_mut()
                .for_each(|sample| *sample = 0.0);
            muffler_element
                .chamber1
                .samples
                .data
                .iter_mut()
                .for_each(|sample| *sample = 0.0);
        }

        self.engine.exhaust_collector = 0.0;
        self.engine.intake_collector = 0.0;
    }

    pub fn frame(&mut self) -> f32 {
        let inc = self.engine.rpm / (self.samples_per_second as f32 * 120.0);
        self.engine.crankshaft_pos = (self.engine.crankshaft_pos + inc).fract();
        let (intake, vibration, exhaust) = self.gen();
        let mixed = exhaust.madd(
            self.engine.exhaust_volume,
            intake.madd(
                self.engine.intake_volume,
                vibration * self.engine.engine_vibrations_volume,
            ),
        ) * self.volume;

        // reduces dc offset
        mixed - self.dc_lp.filter(mixed)
    }

    /// generates one sample worth of audio
    /// returns  `(intake, engine vibrations, exhaust, waveguides dampened)`
    fn gen(&mut self) -> (f32, f32, f32) {
        let intake_noise = self
            .engine
            .intake_noise_lp
            .filter(self.engine.intake_noise.step())
            * self.engine.intake_noise_factor;

        let mut engine_vibration = 0.0;

        let num_cyl = self.engine.cylinders.len() as f32;

        let last_exhaust_collector = self.engine.exhaust_collector / num_cyl;
        self.engine.exhaust_collector = 0.0;
        self.engine.intake_collector = 0.0;

        let crankshaft_fluctuation_offset = self
            .engine
            .crankshaft_fluctuation_lp
            .filter(self.engine.crankshaft_noise.step());

        for cylinder in self.engine.cylinders.iter_mut() {
            let (cyl_intake, cyl_exhaust, cyl_vib) = cylinder.pop(
                self.engine
                    .crankshaft_fluctuation
                    .madd(crankshaft_fluctuation_offset, self.engine.crankshaft_pos),
                last_exhaust_collector,
                self.engine.intake_valve_shift,
                self.engine.exhaust_valve_shift,
            );

            self.engine.intake_collector += cyl_intake;
            self.engine.exhaust_collector += cyl_exhaust;

            engine_vibration += cyl_vib;
        }

        // parallel input to the exhaust straight pipe
        // alpha end is at exhaust collector
        let straight_pipe_wg_ret = self.engine.muffler.straight_pipe.pop();

        // alpha end is at straight pipe end (beta)
        let mut muffler_wg_ret = (0.0, 0.0);

        for muffler_line in self.engine.muffler.muffler_elements.iter_mut() {
            let ret = muffler_line.pop();
            muffler_wg_ret.0 += ret.0;
            muffler_wg_ret.1 += ret.1;
        }

        // pop  //
        //////////
        // push //

        for cylinder in self.engine.cylinders.iter_mut() {
            // modulate intake
            cylinder.push(intake_noise.madd(
                intake_valve((self.engine.crankshaft_pos + cylinder.crank_offset).fract()),
                self.engine.intake_collector / num_cyl,
            ));
        }

        self.engine
            .muffler
            .straight_pipe
            .push(self.engine.exhaust_collector, muffler_wg_ret.0);

        self.engine.exhaust_collector += straight_pipe_wg_ret.0;

        let muffler_elements = self.engine.muffler.muffler_elements.len() as f32;

        for muffler_delay_line in self.engine.muffler.muffler_elements.iter_mut() {
            muffler_delay_line.push(straight_pipe_wg_ret.1 / muffler_elements, 0.0);
        }

        engine_vibration = self.engine.engine_vibration_filter.filter(engine_vibration);

        (
            self.engine.intake_collector,
            engine_vibration,
            muffler_wg_ret.1,
        )
    }
}

#[derive(Clone, Default)]
pub struct WaveGuide {
    // goes from x0 to x1
    pub chamber0: DelayLine,
    // goes from x1 to x0
    pub chamber1: DelayLine,
    /// reflection factor for the first value of the return tuple of `pop`
    pub alpha: f32,
    /// reflection factor for the second value of the return tuple of `pop`
    pub beta: f32,

    // running values
    // #[serde(skip)]
    c1_out: f32,
    // #[serde(skip)]
    c0_out: f32,
}

impl WaveGuide {
    #[inline]
    pub fn new(delay: usize, alpha: f32, beta: f32, samples_per_second: u32) -> Self {
        Self {
            chamber0: DelayLine::new(delay, samples_per_second),
            chamber1: DelayLine::new(delay, samples_per_second),
            alpha,
            beta,
            c1_out: 0.0,
            c0_out: 0.0,
        }
    }

    #[inline]
    pub fn pop(&mut self) -> (f32, f32) {
        self.c1_out = Self::dampen(self.chamber1.pop());
        self.c0_out = Self::dampen(self.chamber0.pop());

        (
            self.c1_out * (1.0 - self.alpha.abs()),
            self.c0_out * (1.0 - self.beta.abs()),
        )
    }

    #[inline]
    pub fn dampen(sample: f32) -> f32 {
        let sample_abs = sample.abs();
        if sample_abs > WAVEGUIDE_MAX_AMP {
            sample.signum()
                * (-1.0 / (sample_abs - WAVEGUIDE_MAX_AMP + 1.0) + 1.0 + WAVEGUIDE_MAX_AMP)
        } else {
            sample
        }
    }

    #[inline]
    pub fn push(&mut self, x0_in: f32, x1_in: f32) {
        let c0_in = self.c1_out.madd(self.alpha, x0_in);
        let c1_in = self.c0_out.madd(self.beta, x1_in);

        self.chamber0.push(c0_in);
        self.chamber1.push(c1_in);
        self.chamber0.samples.advance();
        self.chamber1.samples.advance();
    }
}

#[derive(Clone, Default)]
pub struct LoopBuffer {
    // in seconds
    pub delay: f32,
    // #[serde(skip)]
    pub data: Box<[f32]>,
    // #[serde(skip)]
    pub pos: usize,
}

impl LoopBuffer {
    /// Creates a new loop buffer with specifies length.
    /// The internal sample buffer size is rounded up to the currently best SIMD implementation's float vector size.
    pub fn new(len: usize, samples_per_second: u32) -> Self {
        Self {
            delay: len as f32 / samples_per_second as f32,
            data: vec![0.0; len].into(),
            pos: 0,
        }
    }

    /// Sets the value at the current position. Must be called with `pop`.
    /// ```rust
    /// let mut lb = LoopBuffer::new(2);
    /// lb.push(1.0);
    /// lb.advance();
    ///
    /// assert_eq(lb.pop(), 1.0);
    ///
    /// ```
    pub fn push(&mut self, value: f32) {
        let len = self.data.len();
        self.data[self.pos % len] = value;
    }

    /// Gets the value `self.len` samples prior. Must be called with `push`.
    /// See `push` for examples
    pub fn pop(&mut self) -> f32 {
        let len = self.data.len();
        self.data[(self.pos + 1) % len]
    }

    /// Advances the position of this loop buffer.
    pub fn advance(&mut self) {
        self.pos += 1;
    }
}

#[derive(Clone, Default)]
pub struct LowPassFilter {
    /// 1 / cutoff frequency
    pub delay: f32,
    // #[serde(skip)]
    pub alpha: f32,
    // #[serde(skip)]
    pub last: f32,
}

impl LowPassFilter {
    pub fn new(freq: f32, samples_per_second: u32) -> Self {
        Self {
            delay: 1.0 / freq,
            alpha: (PI2F * (1.0 / samples_per_second as f32) * freq)
                / (PI2F * (1.0 / samples_per_second as f32)).madd(freq, 1.0),
            last: 0.0,
        }
    }

    #[inline]
    pub fn filter(&mut self, sample: f32) -> f32 {
        let ret = (sample - self.last).madd(self.alpha, self.last);
        self.last = ret;
        ret
    }
}

#[derive(Clone, Default)]
pub struct DelayLine {
    pub samples: LoopBuffer,
}

impl DelayLine {
    pub fn new(delay: usize, samples_per_second: u32) -> Self {
        Self {
            samples: LoopBuffer::new(delay, samples_per_second),
        }
    }

    pub fn pop(&mut self) -> f32 {
        self.samples.pop()
    }

    pub fn push(&mut self, sample: f32) {
        self.samples.push(sample);
    }
}

fn exhaust_valve(crank_pos: f32) -> f32 {
    if 0.75 < crank_pos && crank_pos < 1.0 {
        (-(crank_pos * PI4F)).sinf()
    } else {
        0.0
    }
}

fn intake_valve(crank_pos: f32) -> f32 {
    if 0.0 < crank_pos && crank_pos < 0.25 {
        (crank_pos * PI4F).sinf()
    } else {
        0.0
    }
}

fn piston_motion(crank_pos: f32) -> f32 {
    (crank_pos * PI4F).cosf()
}

fn fuel_ignition(crank_pos: f32, ignition_time: f32) -> f32 {
    /*if 0.0 < crank_pos && crank_pos < ignition_time {
        sin(PI2F * (crank_pos * ignition_time + 0.5))
    } else {
        0.0
    }*/
    if 0.5 < crank_pos && crank_pos < ignition_time / 2.0 + 0.5 {
        (PI2F * ((crank_pos - 0.5) / ignition_time)).sinf()
    } else {
        0.0
    }
}
