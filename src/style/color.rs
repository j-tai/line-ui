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

#[allow(missing_docs)]
impl Color {
    pub const BLACK: Color = Color::Ansi(0);
    pub const RED: Color = Color::Ansi(1);
    pub const GREEN: Color = Color::Ansi(2);
    pub const YELLOW: Color = Color::Ansi(3);
    pub const BLUE: Color = Color::Ansi(4);
    pub const MAGENTA: Color = Color::Ansi(5);
    pub const CYAN: Color = Color::Ansi(6);
    pub const WHITE: Color = Color::Ansi(7);
    pub const LIGHT_BLACK: Color = Color::Ansi(8);
    pub const LIGHT_RED: Color = Color::Ansi(9);
    pub const LIGHT_GREEN: Color = Color::Ansi(10);
    pub const LIGHT_YELLOW: Color = Color::Ansi(11);
    pub const LIGHT_BLUE: Color = Color::Ansi(12);
    pub const LIGHT_MAGENTA: Color = Color::Ansi(13);
    pub const LIGHT_CYAN: Color = Color::Ansi(14);
    pub const LIGHT_WHITE: Color = Color::Ansi(15);
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
