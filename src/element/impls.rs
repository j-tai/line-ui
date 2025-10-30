/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use crate::element::Element;
use crate::render::RenderChunk;

impl<'s, E: Element<'s> + ?Sized> Element<'s> for &'_ E {
    fn width(&self) -> usize {
        (*self).width()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        (*self).render()
    }
}

impl<'s, E: Element<'s>> Element<'s> for [E] {
    fn width(&self) -> usize {
        self.iter().map(|e| e.width()).sum()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl<'s, E: Element<'s>, const N: usize> Element<'s> for [E; N] {
    fn width(&self) -> usize {
        self.iter().map(|e| e.width()).sum()
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl<'s, E: Element<'s>> Element<'s> for Option<E> {
    fn width(&self) -> usize {
        match self {
            Some(inner) => inner.width(),
            None => 0,
        }
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        self.iter().flat_map(|e| e.render())
    }
}

impl<'s> Element<'s> for () {
    fn width(&self) -> usize {
        0
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        std::iter::empty()
    }
}

macro_rules! impl_element_for_tuple {
    ( A $( $t:ident )* , 0 $( $n:tt )* ) => {
        impl<'s, A $(, $t)*> Element<'s> for (A, $($t),*)
        where
            A: Element<'s>,
            $($t: Element<'s>,)*
        {
            fn width(&self) -> usize {
                self.0.width()
                $(+ self.$n.width())*
            }

            fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
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
impl<'s, L: Element<'s>, R: Element<'s>> Element<'s> for either::Either<L, R> {
    fn width(&self) -> usize {
        either::for_both!(self, inner => inner.width())
    }

    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>> {
        self.as_ref().map_either(L::render, R::render)
    }
}

#[cfg(test)]
mod tests {
    use crate::element::{Gap, IntoElement};

    use super::*;

    fn is_element<'s, E: Element<'s> + ?Sized>() {}
    fn value_is_element<'s, E: Element<'s> + ?Sized>(_: &E) {}

    #[test]
    fn element_impls() {
        is_element::<[Gap]>();
        is_element::<&[Gap]>();
        is_element::<[Gap; 42]>();
        is_element::<[&[Gap]; 4]>();
        is_element::<((), (), (), (), (), (), (), Gap, (), ())>();

        let not_static = "foo".to_owned();
        let not_static = not_static[..].into_element();
        value_is_element(&not_static);
        value_is_element(&(not_static, Gap(1)));

        #[cfg(feature = "either")]
        is_element::<either::Either<Gap, (Gap, Gap)>>();
    }

    fn same_lifetime<'s, L: Element<'s>, R: Element<'s>>(_: &L, _: &R) {}

    #[test]
    fn element_lifetime() {
        let not_static = "foo".to_owned();
        let mut not_static = not_static[..].into_element();
        let is_static = "foo".into_element();
        same_lifetime(&is_static, &not_static);
        same_lifetime(&not_static, &is_static);
        not_static = is_static;
        let _ = not_static;
    }
}
