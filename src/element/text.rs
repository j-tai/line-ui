/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::Style;
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
            width: crate::width(value),
        }
    }
}

impl<'s> From<&'s str> for Text<'s> {
    fn from(value: &'s str) -> Self {
        Text::new(value)
    }
}

impl<'s> Element<'s> for Text<'s> {
    fn width(&self) -> usize {
        self.width
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        std::iter::once(RenderChunk::with_known_width(
            self.value,
            self.width,
            Style::EMPTY,
        ))
    }
}

#[test]
fn basic() {
    let element = Text::from("hello");
    let render: Vec<_> = element.render().collect();
    assert_eq!(render, ["hello".into()])
}
