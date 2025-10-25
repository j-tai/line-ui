/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::Element;
use crate::render::RenderChunk;

/// An element that places the cursor at its position.
#[derive(Debug, Clone, Copy)]
pub struct Cursor;

impl Element for Cursor {
    fn width(&self) -> usize {
        0
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        std::iter::once(RenderChunk::CURSOR)
    }
}
