/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::{Element, FixedWidth, Styled, Text};
use crate::style::Style;

/// A type that can be converted into an element.
pub trait IntoElement: Sized {
    /// The element type to be converted into.
    type ElementType: Element;

    /// Converts this type into an [`Element`].
    fn into_element(self) -> Self::ElementType;

    /// Convenience function to wrap this element in a [`FixedWidth`].
    fn fixed_width(self, width: usize) -> FixedWidth<Self::ElementType> {
        FixedWidth::new(width, self.into_element())
    }

    /// Convenience function to wrap this element in a [`Styled`].
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
