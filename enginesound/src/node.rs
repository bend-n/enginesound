use crate::gen::{Generator, LowPassFilter};
use crate::utils::*;
use godot::engine::{
    AudioStreamGenerator, AudioStreamGeneratorPlayback, AudioStreamGeneratorVirtual,
};
use godot::prelude::*;
const DEFAULT_CONFIG: &[u8] = include_bytes!("default.esc");

type Stream = Gd<AudioStreamGeneratorPlayback>;

/// Procedural engine sound generation
#[derive(GodotClass)]
#[class(base=AudioStreamGenerator)]
#[allow(dead_code)]
pub struct EngineStream {
    generator: Option<Generator>,
    stream: Option<Stream>,
    #[export(get, set)]
    engine_rpm: f32,
    #[export(get, set)]
    engine_volume: f32,
    #[base]
    base: Base<AudioStreamGenerator>,
}

#[godot_api]
impl AudioStreamGeneratorVirtual for EngineStream {
    fn init(base: Base<AudioStreamGenerator>) -> Self {
        Self {
            generator: None,
            base,
            stream: None,
            engine_rpm: 883.0,
            engine_volume: 1.0,
        }
    }
}

macro_rules! fail_cond {
    ($cond:expr, $err:expr) => {
        if $cond {
            godot_error!($err);
            return;
        }
    };
    ($cond:expr, $err:expr, $retval:expr) => {
        if $cond {
            godot_error!($err);
            return $retval;
        }
    };
}

#[godot_api]
impl EngineStream {
    /// Creates a generator.
    #[func]
    fn make_engine(&mut self) {
        let mut engine = ron::de::from_bytes(DEFAULT_CONFIG).expect("default config is invalid");
        fix_engine(&mut engine, self.get_mix_rate() as u32);
        let mut generator = Generator::new(
            self.get_mix_rate() as u32,
            engine,
            LowPassFilter::new(0.5, self.get_mix_rate() as u32),
        );
        generator.volume = 1.0;
        self.generator = Some(generator)
    }

    /// Fills the [AudioStreamGeneratorPlayback]'s buffer.
    #[func]
    fn update(&mut self) {
        fail_cond!(self.stream.is_none(), "No stream!");
        if self.generator.is_none() {
            self.make_engine();
        }
        let gen = self.generator.as_mut().unwrap();
        gen.volume = self.engine_volume;
        gen.engine.rpm = self.engine_rpm;
        gen.generate(self.stream.as_mut().unwrap());
    }

    /// Sets the [AudioStreamGeneratorPlayback] for this engine.
    #[func]
    fn set_stream(&mut self, stream: Stream) {
        self.stream = Some(stream);
    }
}
