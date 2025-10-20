/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::{Element, FixedWidth, Styled, Text};
use crate::style::Style;

pub trait IntoElement: Sized {
    type ElementType: Element;

    fn into_element(self) -> Self::ElementType;

    fn fixed_width(self, width: usize) -> FixedWidth<Self::ElementType> {
        FixedWidth::new(width, self.into_element())
    }

    fn with_style(self, style: Style) -> Styled<Self::ElementType> {
        Styled::new(style, self.into_element())
    }
}

impl<E: Element> IntoElement for E {
    type ElementType = Self;

    fn into_element(self) -> Self::ElementType {
        self
    }
}

impl<'s> IntoElement for &'s str {
    type ElementType = Text<'s>;

    fn into_element(self) -> Self::ElementType {
        Text::from(self)
    }
}
