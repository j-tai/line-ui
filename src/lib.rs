#![doc = include_str!("../README.md")]

pub mod element;
mod render;
mod style;

pub use render::Renderer;
pub use style::*;
