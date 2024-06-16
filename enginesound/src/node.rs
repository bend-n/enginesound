use std::cell::OnceCell;

use crate::gen::{Engine, Generator, LowPassFilter};
use godot::engine::{AudioStreamGenerator, AudioStreamGeneratorPlayback, IAudioStreamGenerator};
use godot::prelude::*;

type Stream = Gd<AudioStreamGeneratorPlayback>;

/// Procedural engine sound generation
#[derive(GodotClass)]
#[class(base=AudioStreamGenerator)]
pub struct EngineStream {
    /// if this was set in init() the mix rate would be wrong
    generator: OnceCell<Generator>,
    stream: OnceCell<Stream>,
    /// The rotations per minute of the engine.
    #[var]
    engine_rpm: f32,
    base: Base<AudioStreamGenerator>,
}

#[godot_api]
impl IAudioStreamGenerator for EngineStream {
    fn init(base: Base<AudioStreamGenerator>) -> Self {
        Self {
            generator: OnceCell::new(),
            base,
            stream: OnceCell::new(),
            engine_rpm: 883.0,
        }
    }
}

#[godot_api]
impl EngineStream {
    /// Fills the [AudioStreamGeneratorPlayback]'s buffer.
    #[func]
    fn update(&mut self) {
        let b = &self.to_gd();
        let gen = self.generator.get_mut_or_init(|| {
            let sps = b.get_mix_rate() as u32;
            if sps == 0 {
                godot_error!("0 samples?");
                unreachable!();
            }
            Generator::new(sps, Engine::new(sps), LowPassFilter::new(0.5, sps))
        });
        let Some(stream) = self.stream.get_mut() else {
            return godot_error!("No stream! call `set_stream` first.");
        };
        gen.engine.rpm = self.engine_rpm;
        gen.generate(stream);
    }

    /// Sets the [AudioStreamGeneratorPlayback] for this engine.
    #[func]
    fn set_stream(&mut self, stream: Stream) {
        self.stream.get_or_init(|| stream);
    }
}
