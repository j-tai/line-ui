/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use std::collections::VecDeque;

use unicode_width::UnicodeWidthStr;

use crate::element::{Element, Gap};
use crate::render::RenderChunk;

/// An element that pads or truncates its contents to a constant width.
#[derive(Debug, Clone)]
pub struct FixedWidth<E> {
    width: usize,
    truncate: Direction,
    pad: Direction,
    content: E,
}

impl<E> FixedWidth<E> {
    /// Creates a new [`FixedWidth`] with the specified width and content.
    pub fn new(width: usize, content: E) -> Self {
        FixedWidth {
            width,
            truncate: Direction::Right,
            pad: Direction::Right,
            content,
        }
    }

    /// Changes the side on which the content is truncated.
    ///
    /// This option only takes effect if the content is wider than the width.
    pub fn truncated(mut self, truncate: Direction) -> Self {
        self.truncate = truncate;
        self
    }

    /// Changes the side on which padding is added.
    ///
    /// This option only takes effect if the content is narrower than the width.
    pub fn padded(mut self, pad: Direction) -> Self {
        self.pad = pad;
        self
    }

    fn render_impl<'s>(
        &'s self,
        content: impl DoubleEndedIterator<Item = RenderChunk<'s>>,
        truncate: impl for<'t> Fn(RenderChunk<'t>, usize) -> RenderChunk<'t>,
    ) -> (Vec<RenderChunk<'s>>, Gap) {
        let mut accumulated_width = 0;
        let mut result = Vec::new();

        for item in content {
            let item_width = item.width;
            let available_width = self.width - accumulated_width;
            if item_width > available_width {
                if available_width > 0 {
                    let truncated_item = truncate(item, available_width);
                    accumulated_width += truncated_item.width;
                    result.push(truncated_item);
                }
                break;
            } else {
                accumulated_width += item.width;
                result.push(item);
            }
        }

        (result, Gap(self.width - accumulated_width))
    }
}

impl<E: Element> Element for FixedWidth<E> {
    fn width(&self) -> usize {
        self.width
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        let (result, gap) = match self.truncate {
            Direction::Left => {
                let (mut result, gap) =
                    self.render_impl(self.content.render().rev(), truncate_start);
                result.reverse();
                (result, gap)
            }
            Direction::Right => self.render_impl(self.content.render(), truncate_end),
        };
        let mut result = VecDeque::from(result);

        match self.pad {
            Direction::Left => {
                for chunk in gap.into_render() {
                    result.push_front(chunk);
                }
            }
            Direction::Right => {
                for chunk in gap.into_render() {
                    result.push_back(chunk);
                }
            }
        }

        result.into_iter()
    }
}

fn truncate_end<'s>(input: RenderChunk<'s>, target: usize) -> RenderChunk<'s> {
    let mut best_index = 0;
    let mut best_width = 0;

    for (index, _) in input.value.char_indices().skip(1) {
        let width = input.value[..index].width();
        if width <= target {
            best_index = index;
            best_width = width;
        } else {
            break;
        }
    }

    debug_assert!(best_width <= target);
    RenderChunk::with_known_width(&input.value[..best_index], best_width, input.style)
}

fn truncate_start<'s>(input: RenderChunk<'s>, target: usize) -> RenderChunk<'s> {
    let mut best_index = input.value.len();
    let mut best_width = 0;

    for (index, _) in input.value.char_indices().rev() {
        let width = input.value[index..].width();
        if width <= target {
            best_index = index;
            best_width = width;
        } else {
            break;
        }
    }

    debug_assert!(best_width <= target);
    RenderChunk::with_known_width(&input.value[best_index..], best_width, input.style)
}

/// The alignment or padding applied to a [`FixedWidth`] element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    /// Left (start).
    #[default]
    Left,
    /// Right (end).
    Right,
}

#[cfg(test)]
mod tests {
    use crate::element::{Cursor, IntoElement, Text};

    use super::*;

    #[test]
    fn width_zero() {
        let element = "hello".fixed_width(0);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, []);
    }

    #[test]
    fn empty_content() {
        let element = ().fixed_width(4);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, [RenderChunk::from("    ")]);
    }

    #[test]
    fn blank_chunks_do_not_drop_cursor() {
        let element = (Text::from(""), Text::from(""), Cursor).fixed_width(0);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["".into(), "".into(), RenderChunk::CURSOR]);
    }

    #[test]
    fn blank_content() {
        let element = "".fixed_width(5);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["", "     "].map(RenderChunk::from));
    }

    #[test]
    fn short_content() {
        let element = "foo".fixed_width(6);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foo", "   "].map(RenderChunk::from));
    }

    #[test]
    fn equal_content() {
        let element = "foobar".fixed_width(6);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobar"].map(RenderChunk::from));
    }

    #[test]
    fn long_content() {
        let element = "foobarbaz".fixed_width(8);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobarba"].map(RenderChunk::from));
    }

    #[test]
    fn long_content_with_more() {
        let element = (Text::from("foobarbaz"), Text::from("asdf")).fixed_width(8);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobarba"].map(RenderChunk::from));
    }

    #[test]
    fn short_content_truncated_left() {
        let element = "foo".fixed_width(6).truncated(Direction::Left);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foo", "   "].map(RenderChunk::from));
    }

    #[test]
    fn equal_content_truncated_left() {
        let element = "foobar".fixed_width(6).truncated(Direction::Left);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobar"].map(RenderChunk::from));
    }

    #[test]
    fn long_content_truncated_left() {
        let element = "foobarbaz".fixed_width(8).truncated(Direction::Left);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["oobarbaz"].map(RenderChunk::from));
    }

    #[test]
    fn long_content_with_more_truncated_left() {
        let element = (Text::from("asdf"), Text::from("foobarbaz"))
            .fixed_width(8)
            .truncated(Direction::Left);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["oobarbaz"].map(RenderChunk::from));
    }

    #[test]
    fn short_content_padded_left() {
        let element = "foo".fixed_width(6).padded(Direction::Left);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["   ", "foo"].map(RenderChunk::from));
    }
}
