use crate::gen::{Generator, LowPassFilter};
use crate::utils::*;
use godot::engine::{AudioStreamGeneratorPlayback, Node};
use godot::prelude::*;
const DEFAULT_CONFIG: &[u8] = include_bytes!("default.esc");

type Stream = Gd<AudioStreamGeneratorPlayback>;

/// Procedural engine sound generation
#[derive(GodotClass)]
#[class(base=Node)]
pub struct EngineNoise {
    generator: Generator,
    #[property]
    stream: Option<Stream>,
    #[base]
    base: Base<Node>,
}
const SAMPLE_RATE: u32 = 30000;

#[godot_api]
impl GodotExt for EngineNoise {
    fn init(base: Base<Node>) -> Self {
        let mut engine = ron::de::from_bytes(DEFAULT_CONFIG).expect("default config is invalid");
        fix_engine(&mut engine, SAMPLE_RATE);
        let mut generator =
            Generator::new(SAMPLE_RATE, engine, LowPassFilter::new(0.5, SAMPLE_RATE));
        generator.volume = 1.0;
        Self {
            generator,
            base,
            stream: None,
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
impl EngineNoise {
    /// Change the rpm of the engine
    #[func]
    fn set_rpm(&mut self, rpm: f32) {
        self.generator.engine.rpm = rpm;
    }

    #[func]
    fn update(&mut self) {
        fail_cond!(self.stream.is_none(), "No stream!");
        self.generator.generate(&mut self.stream.as_mut().unwrap());
    }

    #[func]
    fn set_stream(&mut self, stream: Stream) {
        self.stream = Some(stream);
        self.update();
    }

    #[func]
    fn set_volume(&mut self, v: f32) {
        self.generator.volume = v
    }
}
