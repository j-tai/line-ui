/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use unicode_width::UnicodeWidthStr;

use crate::element::{Element, Gap};
use crate::render::RenderChunk;

/// An element that pads or truncates its contents to a constant width.
#[derive(Debug, Clone)]
pub struct FixedWidth<E> {
    width: usize,
    content: E,
}

impl<E> FixedWidth<E> {
    /// Creates a new [`FixedWidth`] with the specified width and content.
    pub fn new(width: usize, content: E) -> Self {
        FixedWidth { width, content }
    }
}

impl<E: Element> Element for FixedWidth<E> {
    fn width(&self) -> usize {
        self.width
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        let mut result = Vec::new();
        let mut accumulated_width = 0;

        if self.width == 0 {
            return result.into_iter();
        }

        for item in self.content.render() {
            let item_width = item.width;
            let available_width = self.width - accumulated_width;
            if item_width > available_width {
                let truncated_item = truncate_end(item, available_width);
                accumulated_width += truncated_item.width;
                result.push(truncated_item);
                break;
            } else {
                accumulated_width += item.width;
                result.push(item);
            }
        }

        for item in Gap(self.width - accumulated_width).into_render() {
            result.push(item);
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

#[cfg(test)]
mod tests {
    use crate::element::Text;

    use super::*;

    #[test]
    fn width_zero() {
        let element = FixedWidth {
            width: 0,
            content: Text::from("hello"),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, []);
    }

    #[test]
    fn empty_content() {
        let element = FixedWidth {
            width: 4,
            content: (),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, [RenderChunk::from("    ")]);
    }

    #[test]
    fn blank_content() {
        let element = FixedWidth {
            width: 5,
            content: Text::from(""),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["", "     "].map(RenderChunk::from));
    }

    #[test]
    fn short_content() {
        let element = FixedWidth {
            width: 6,
            content: Text::from("foo"),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foo", "   "].map(RenderChunk::from));
    }

    #[test]
    fn equal_content() {
        let element = FixedWidth {
            width: 6,
            content: Text::from("foobar"),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobar"].map(RenderChunk::from));
    }

    #[test]
    fn long_content() {
        let element = FixedWidth {
            width: 8,
            content: Text::from("foobarbaz"),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobarba"].map(RenderChunk::from));
    }

    #[test]
    fn long_content_with_more() {
        let element = FixedWidth {
            width: 8,
            content: (Text::from("foobarbaz"), Text::from("asdf")),
        };
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["foobarba"].map(RenderChunk::from));
    }
}
