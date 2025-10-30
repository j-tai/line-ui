/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::{BoxElement, Element, FixedWidth, Styled, Text};
use crate::style::Style;

/// A type that can be converted into an element.
pub trait IntoElement<'s>: Sized {
    /// The element type to be converted into.
    type ElementType: Element<'s>;

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

    /// Convenience function to box this element.
    fn boxed(self) -> BoxElement<'s> {
        BoxElement::new(self.into_element())
    }
}

impl<'s, E: Element<'s>> IntoElement<'s> for E {
    type ElementType = Self;

    fn into_element(self) -> Self::ElementType {
        self
    }
}

impl<'s> IntoElement<'s> for &'s str {
    type ElementType = Text<'s>;

    fn into_element(self) -> Self::ElementType {
        Text::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::Renderer;

    use super::*;

    #[test]
    fn non_static_lifetime() {
        let string = "foo".to_owned();
        let not_static = string[..].into_element();

        let mut r = Renderer::new(vec![]);
        let _ = r.render(&not_static);
        let _ = r.render(not_static.fixed_width(42));
        let _ = r.finish();
    }
}
