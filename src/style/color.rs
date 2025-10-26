/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

/// A terminal color, which can be applied to the foreground or background.
///
/// Note that some distinct values of [`Color`] could end up appearing as the
/// same color when displayed in the terminal. However, the [`PartialEq`]
/// implementation does not account for this; it only compares the
/// representation of the enum.
///
/// [`From`] implementations are provided for convenience.
///
/// # Example
///
/// ```
/// use line_ui::Color;
///
/// assert_eq!(Color::Default, Color::default());
/// assert_eq!(Color::Ansi(42), 42.into());
/// assert_eq!(Color::Rgb(20, 40, 90), (20, 40, 90).into());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Color {
    /// The default color used by the terminal.
    #[default]
    Default,
    /// An ANSI color code.
    Ansi(u8),
    /// An RGB color.
    Rgb(u8, u8, u8),
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Color::Ansi(value)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::Rgb(r, g, b)
    }
}
