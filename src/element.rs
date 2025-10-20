/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

mod fixed_width;
mod gap;
mod impls;
mod into;
mod styled;
mod text;

use crate::render::RenderChunk;

pub use fixed_width::*;
pub use gap::*;
pub use into::*;
pub use styled::*;
pub use text::*;

pub trait Element {
    fn width(&self) -> usize;

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>>;
}
