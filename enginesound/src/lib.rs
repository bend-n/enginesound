#![feature(once_cell_get_mut)]
pub mod gen;
#[cfg(feature = "godot")]
pub mod node;
pub mod utils;
#[cfg(feature = "godot")]
use godot::prelude::*;

#[cfg(feature = "godot")]
struct Lib;
#[cfg(feature = "godot")]
#[gdextension]
unsafe impl ExtensionLibrary for Lib {}
