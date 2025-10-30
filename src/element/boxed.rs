/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::Element;
use crate::render::RenderChunk;

/// An element with its type erased.
///
/// This type can be used to return multiple kinds of elements from a function.
///
/// # Example
///
/// ```
/// use line_ui::element::{BoxElement, IntoElement};
/// use line_ui::{Color, Style};
///
/// fn foo(value: bool) -> BoxElement<'static> {
///     if value {
///         "some fancy text"
///             .fixed_width(20)
///             .with_style(Style::fg(Color::RED) + Style::INVERT)
///             .boxed()
///     } else {
///         "just some text".boxed()
///     }
/// }
/// ```
pub struct BoxElement<'s> {
    width: usize,
    content: Vec<RenderChunk<'s>>,
}

impl<'s> BoxElement<'s> {
    /// Boxes the provided element.
    pub fn new<E: Element<'s>>(inner: E) -> Self {
        let width = inner.width();
        let content: Vec<_> = inner.render().collect();
        debug_assert_eq!(width, content.iter().map(|chunk| chunk.width).sum());
        BoxElement { width, content }
    }
}

impl<'s> Element<'s> for BoxElement<'s> {
    fn width(&self) -> usize {
        self.width
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        self.content.iter().cloned()
    }
}
