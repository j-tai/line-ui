/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use unicode_width::UnicodeWidthStr;

use crate::element::Element;
use crate::render::RenderChunk;

/// An element that renders a piece of text.
pub struct Text<'s> {
    value: &'s str,
    width: usize,
}

impl<'s> Text<'s> {
    /// Creates a new [`Text`] from the given string.
    pub fn new(value: &'s str) -> Self {
        Text {
            value,
            width: value.width(),
        }
    }
}

impl<'s> From<&'s str> for Text<'s> {
    fn from(value: &'s str) -> Self {
        Text::new(value)
    }
}

impl Element for Text<'_> {
    fn width(&self) -> usize {
        self.width
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        std::iter::once(RenderChunk::from(self.value))
    }
}

#[test]
fn basic() {
    let element = Text::from("hello");
    let render: Vec<_> = element.render().collect();
    assert_eq!(render, ["hello".into()])
}
