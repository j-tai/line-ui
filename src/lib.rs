#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod element;
mod render;
mod style;

pub use render::Renderer;
pub use style::*;
