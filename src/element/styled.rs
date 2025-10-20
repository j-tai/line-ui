/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::Element;
use crate::render::RenderChunk;
use crate::style::Style;

/// An element that renders its content with a particular style.
pub struct Styled<E> {
    style: Style,
    inner: E,
}

impl<E> Styled<E> {
    /// Creates a new [`Styled`].
    pub fn new(style: Style, inner: E) -> Self {
        Styled { style, inner }
    }
}

impl<E: Element> Element for Styled<E> {
    fn width(&self) -> usize {
        self.inner.width()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        self.inner.render().map(|mut item| {
            item.style = item.style.or(self.style);
            item
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::element::Text;

    use super::*;

    const STYLE_1: Style = Style {
        foreground: Some(42),
        ..Style::EMPTY
    };

    const STYLE_2: Style = Style {
        foreground: Some(96),
        ..Style::EMPTY
    };

    const STYLE_3: Style = Style {
        background: Some(1),
        ..Style::EMPTY
    };

    #[test]
    fn basic() {
        let element = Styled::new(STYLE_1, Text::from("Hello, world!"));
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, [RenderChunk::new("Hello, world!", STYLE_1)]);
    }

    #[test]
    fn nested() {
        let element = Styled::new(STYLE_1, Styled::new(STYLE_2, Text::from("Hello, world!")));
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, [RenderChunk::new("Hello, world!", STYLE_2)]);
    }

    #[test]
    fn nested_merge() {
        let element = Styled::new(STYLE_3, Styled::new(STYLE_2, Text::from("Hello, world!")));
        let render: Vec<_> = element.render().collect();
        assert_eq!(
            render,
            [RenderChunk::new(
                "Hello, world!",
                Style {
                    foreground: Some(96),
                    background: Some(1),
                    ..Style::EMPTY
                },
            )],
        );
    }
}
