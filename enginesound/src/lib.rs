pub const DEFAULT_CONFIG: &[u8] = include_bytes!("default.esc");

pub mod gen;
pub mod node;
pub mod utils;
pub use godot::prelude::*;

struct Lib;
#[gdextension]
unsafe impl ExtensionLibrary for Lib {}
