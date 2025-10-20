/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use std::marker::PhantomData;

use crate::Style;
use crate::element::Element;
use crate::render::RenderChunk;

#[derive(Debug, Clone, Copy)]
pub struct Gap(pub usize);

impl Gap {
    pub(crate) fn into_render<'s>(self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        GapIter {
            size: self.0,
            phantom: PhantomData,
        }
    }
}

impl Element for Gap {
    fn width(&self) -> usize {
        self.0
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        GapIter {
            size: self.0,
            phantom: PhantomData,
        }
    }
}

struct GapIter<'s> {
    size: usize,
    phantom: PhantomData<&'s ()>,
}

// I could just set Item = StyledStr<'static>, but the compiler doesn't like
// that for some reason
impl<'s> Iterator for GapIter<'s> {
    type Item = RenderChunk<'s>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let value = self.size.div_ceil(GAP.len());
        (value, Some(value))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            let spaces = self.size.min(GAP.len());
            self.size -= spaces;
            Some(RenderChunk {
                value: &GAP[..spaces],
                width: spaces,
                style: Style::EMPTY,
                cursor: None,
            })
        }
    }
}

impl<'s> DoubleEndedIterator for GapIter<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

const GAP: &str = "                "; // 16 spaces

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let element = Gap(0);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, []);
    }

    #[test]
    fn short() {
        let element = Gap(7);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, ["       ".into()]);
    }

    #[test]
    fn long() {
        let element = Gap(GAP.len() + 2);
        let render: Vec<_> = element.render().collect();
        assert_eq!(render, [GAP.into(), "  ".into()]);
    }
}
