mod node;

pub mod gen;
pub mod utils;
pub use godot::prelude::*;

struct Lib;
#[gdextension]
unsafe impl ExtensionLibrary for Lib {}
