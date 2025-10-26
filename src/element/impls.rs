/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::Element;
use crate::render::RenderChunk;

impl<E: Element + ?Sized> Element for &'_ E {
    fn width(&self) -> usize {
        (*self).width()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        (*self).render()
    }
}

impl<E: Element> Element for [E] {
    fn width(&self) -> usize {
        self.iter().map(|e| e.width()).sum()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl<E: Element, const N: usize> Element for [E; N] {
    fn width(&self) -> usize {
        self.iter().map(|e| e.width()).sum()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl<E: Element> Element for Option<E> {
    fn width(&self) -> usize {
        match self {
            Some(inner) => inner.width(),
            None => 0,
        }
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl Element for () {
    fn width(&self) -> usize {
        0
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        std::iter::empty()
    }
}

macro_rules! impl_element_for_tuple {
    ( A $( $t:ident )* , 0 $( $n:tt )* ) => {
        impl<A $(, $t)*> Element for (A, $($t),*)
        where
            A: Element,
            $($t: Element,)*
        {
            fn width(&self) -> usize {
                self.0.width()
                $(+ self.$n.width())*
            }

            fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
                self.0.render()
                $(.chain(self.$n.render()))*
            }
        }
    };
}

impl_element_for_tuple!(A, 0);
impl_element_for_tuple!(A B, 0 1);
impl_element_for_tuple!(A B C, 0 1 2);
impl_element_for_tuple!(A B C D, 0 1 2 3);
impl_element_for_tuple!(A B C D E, 0 1 2 3 4);
impl_element_for_tuple!(A B C D E F, 0 1 2 3 4 5);
impl_element_for_tuple!(A B C D E F G, 0 1 2 3 4 5 6);
impl_element_for_tuple!(A B C D E F G H, 0 1 2 3 4 5 6 7);
impl_element_for_tuple!(A B C D E F G H I, 0 1 2 3 4 5 6 7 8);
impl_element_for_tuple!(A B C D E F G H I J, 0 1 2 3 4 5 6 7 8 9);

#[cfg(feature = "either")]
impl<L: Element, R: Element> Element for either::Either<L, R> {
    fn width(&self) -> usize {
        either::for_both!(self, inner => inner.width())
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'_>> {
        self.as_ref().map_either(L::render, R::render)
    }
}

#[cfg(test)]
mod tests {
    use crate::element::Gap;

    use super::*;

    fn is_element<E: Element + ?Sized>() {}

    #[test]
    fn element_impls() {
        is_element::<[Gap]>();
        is_element::<&[Gap]>();
        is_element::<[Gap; 42]>();
        is_element::<[&[Gap]; 4]>();
        is_element::<((), (), (), (), (), (), (), Gap, (), ())>();

        #[cfg(feature = "either")]
        is_element::<either::Either<Gap, (Gap, Gap)>>();
    }
}
