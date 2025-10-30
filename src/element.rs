/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

//! The [`Element`] trait, and various elements.

mod boxed;
mod cursor;
mod fixed_width;
mod gap;
mod impls;
mod into;
mod styled;
mod text;

use crate::render::RenderChunk;

pub use boxed::*;
pub use cursor::*;
pub use fixed_width::*;
pub use gap::*;
pub use into::*;
pub use styled::*;
pub use text::*;

/// A particular widget that can be rendered to the TUI.
///
/// # Lifetime parameter
///
/// The `'s` lifetime parameter indicates the lifetime of any values (e.g.,
/// strings) borrowed by the `Element`. For example:
///
/// ```
/// use line_ui::element::Text;
///
/// let my_string = String::from("hello");
/// let my_element = Text::new(&my_string);
/// ```
///
/// Here, `my_element` implements `Element<'s>`, where `'s` is the lifetime
/// of `my_string`.
pub trait Element<'s> {
    /// The width of the element, in columns.
    fn width(&self) -> usize;

    /// Renders the element into a sequence of chunks.
    fn render(&self) -> impl DoubleEndedIterator<Item = RenderChunk<'s>>;
}
