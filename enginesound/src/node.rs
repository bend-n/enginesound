use crate::gen::{Engine, Generator, LowPassFilter};
use godot::engine::{AudioStreamGenerator, AudioStreamGeneratorPlayback, IAudioStreamGenerator};
use godot::prelude::*;

type Stream = Gd<AudioStreamGeneratorPlayback>;

/// Procedural engine sound generation
#[derive(GodotClass)]
#[class(base=AudioStreamGenerator)]
pub struct EngineStream {
    /// if this was set in init() the mix rate would be wrong
    generator: Option<Generator>,
    stream: Option<Stream>,
    #[var]
    engine_rpm: f32,
    #[export]
    engine_volume: f32,
    base: Base<AudioStreamGenerator>,
}

#[godot_api]
impl IAudioStreamGenerator for EngineStream {
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
}

#[godot_api]
impl EngineStream {
    /// Creates a generator.
    #[func]
    fn make_engine(&mut self) {
        let engine = Engine::new(self.base().get_mix_rate() as u32);
        let mut generator = Generator::new(
            self.base().get_mix_rate() as u32,
            engine,
            LowPassFilter::new(0.5, self.base().get_mix_rate() as u32),
        );
        generator.volume = 1.0;
        self.generator = Some(generator);
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
