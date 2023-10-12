pub mod gen;
pub mod node;
pub mod utils;
use godot::prelude::*;

struct Lib;
#[gdextension]
unsafe impl ExtensionLibrary for Lib {}
