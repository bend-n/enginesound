use crate::gen::{Generator, LowPassFilter};
use crate::utils::*;
use godot::engine::{AudioStreamGeneratorPlayback, Engine, Node};
use godot::prelude::*;
const DEFAULT_CONFIG: &[u8] = include_bytes!("default.esc");

type Stream = Gd<AudioStreamGeneratorPlayback>;

/// Procedural engine sound generation
#[derive(GodotClass)]
#[class(base=Node)]
#[allow(dead_code)]
pub struct EngineNoise {
    generator: Option<Generator>,
    stream: Option<Stream>,
    #[export(get, set)]
    rpm: f32,
    #[export(get, set)]
    volume: f32,
    #[base]
    base: Base<Node>,
}
const SAMPLE_RATE: u32 = 30000;

#[godot_api]
impl NodeVirtual for EngineNoise {
    fn init(base: Base<Node>) -> Self {
        Self {
            generator: None,
            base,
            rpm: 883.0,
            volume: 1.0,
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
    ($cond:expr, $err:expr, $retval:expr) => {
        if $cond {
            godot_error!($err);
            return $retval;
        }
    };
}

macro_rules! require_runtime_some {
    ($retval: expr, $opt: expr) => {
        if Engine::singleton().is_editor_hint() {
            return $retval;
        }
        fail_cond!($opt.is_none(), "Variable unset.", $retval);
    };
}

#[godot_api]
impl EngineNoise {
    #[func]
    fn make_engine(&mut self) {
        let mut engine = ron::de::from_bytes(DEFAULT_CONFIG).expect("default config is invalid");
        fix_engine(&mut engine, SAMPLE_RATE);
        let mut generator =
            Generator::new(SAMPLE_RATE, engine, LowPassFilter::new(0.5, SAMPLE_RATE));
        generator.volume = 1.0;
        self.generator = Some(generator)
    }

    #[func]
    fn update(&mut self) {
        fail_cond!(self.stream.is_none(), "No stream!");
        require_runtime_some!((), self.generator);
        let gen = self.generator.as_mut().unwrap();
        gen.volume = self.volume;
        gen.engine.rpm = self.rpm;
        gen.generate(&mut self.stream.as_mut().unwrap());
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
