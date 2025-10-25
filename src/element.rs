/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

//! The [`Element`] trait, and various elements.

mod cursor;
mod fixed_width;
mod gap;
mod impls;
mod into;
mod styled;
mod text;

use crate::render::RenderChunk;

pub use cursor::*;
pub use fixed_width::*;
pub use gap::*;
pub use into::*;
pub use styled::*;
pub use text::*;

/// A particular widget that can be rendered to the TUI.
pub trait Element {
    /// The width of the element, in columns.
    fn width(&self) -> usize;

    /// Renders the element into a sequence of chunks.
    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>>;
}
