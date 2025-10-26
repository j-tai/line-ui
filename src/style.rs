/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

mod color;

use std::fmt;
use std::ops::{Add, AddAssign};

use termion::color::{AnsiValue, Bg, Fg, Reset, Rgb};

pub use color::Color;

/// A text style, encompassing the color and other style options.
///
/// Each field of this struct is an [`Option`]. When the value is [`None`], then
/// the particular field is *unspecified*. By default, this is the same as
/// setting it to <code>[Some]\([Default::default]\())</code>. However, two
/// style can be *merged* with either [`Style::with`] (or the `+` operator) or
/// [`Style::or`]. This allows another style to override certain fields only if
/// they are unspecified.
///
/// # Example
///
/// ```
/// use line_ui::Style;
///
/// let style1 = Style::fg(1);
/// let style2 = Style::fg(2) + Style::BOLD;
///
/// assert_eq!(style1 + style2, style2);
/// assert_eq!(style1.with(style2), style2); // `with` is equivalent to `+`
///
/// assert_eq!(style2 + style1, Style::fg(1) + Style::BOLD);
/// assert_eq!(style1.or(style2), Style::fg(1) + Style::BOLD);
/// // `or` is equivalent to `+` with operands flipped
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct Style {
    /// The foreground color.
    pub foreground: Option<Color>,
    /// The background color.
    pub background: Option<Color>,
    /// Whether the text should be bold.
    pub bold: Option<bool>,
    /// Whether the text should be italicized.
    pub italic: Option<bool>,
    /// Whether the text should be underlined.
    pub underline: Option<bool>,
    /// Whether the text should be blinking (not widely supported).
    pub blink: Option<bool>,
    /// Whether the text should have its colors inverted.
    pub invert: Option<bool>,
    /// Whether the text should be crossed out (not widely supported).
    pub strikethrough: Option<bool>,
}

impl Style {
    /// The empty style, with nothing specified. Equivalent to `Style::default()`.
    pub const EMPTY: Style = Style {
        foreground: None,
        background: None,
        bold: None,
        italic: None,
        underline: None,
        blink: None,
        invert: None,
        strikethrough: None,
    };

    /// Bold text.
    pub const BOLD: Style = Style {
        bold: Some(true),
        ..Style::EMPTY
    };

    /// Italicized text.
    pub const ITALIC: Style = Style {
        italic: Some(true),
        ..Style::EMPTY
    };

    /// Underlined text.
    pub const UNDERLINE: Style = Style {
        underline: Some(true),
        ..Style::EMPTY
    };

    /// Blinking text (not widely supported).
    pub const BLINK: Style = Style {
        blink: Some(true),
        ..Style::EMPTY
    };

    /// Inverted colors.
    pub const INVERT: Style = Style {
        invert: Some(true),
        ..Style::EMPTY
    };

    /// Crossed-out text (not widely supported).
    pub const STRIKETHROUGH: Style = Style {
        strikethrough: Some(true),
        ..Style::EMPTY
    };

    /// Creates a style with only the foreground specified.
    pub fn fg(color: impl Into<Color>) -> Style {
        Style {
            foreground: Some(color.into()),
            ..Style::EMPTY
        }
    }

    /// Creates a style with only the background specified.
    pub fn bg(color: impl Into<Color>) -> Style {
        Style {
            background: Some(color.into()),
            ..Style::EMPTY
        }
    }

    /// Merges two styles, with `other` taking precedence.
    pub fn with(self, other: Style) -> Style {
        other.or(self)
    }

    /// Merges two styles, with `self` taking precedence.
    pub fn or(self, other: Style) -> Style {
        Style {
            foreground: self.foreground.or(other.foreground),
            background: self.background.or(other.background),
            bold: self.bold.or(other.bold),
            italic: self.italic.or(other.italic),
            underline: self.underline.or(other.underline),
            blink: self.blink.or(other.blink),
            invert: self.invert.or(other.invert),
            strikethrough: self.strikethrough.or(other.strikethrough),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Add for Style {
    type Output = Style;

    fn add(self, rhs: Self) -> Self::Output {
        self.with(rhs)
    }
}

impl AddAssign for Style {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.with(rhs);
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(foreground) = self.foreground {
            match foreground {
                Color::Default => Fg(Reset).fmt(f),
                Color::Ansi(value) => Fg(AnsiValue(value)).fmt(f),
                Color::Rgb(r, g, b) => Fg(Rgb(r, g, b)).fmt(f),
            }?;
        }
        if let Some(background) = self.background {
            match background {
                Color::Default => Bg(Reset).fmt(f),
                Color::Ansi(value) => Bg(AnsiValue(value)).fmt(f),
                Color::Rgb(r, g, b) => Bg(Rgb(r, g, b)).fmt(f),
            }?;
        }
        if self.bold == Some(true) {
            termion::style::Bold.fmt(f)?;
        }
        if self.italic == Some(true) {
            termion::style::Italic.fmt(f)?;
        }
        if self.underline == Some(true) {
            termion::style::Underline.fmt(f)?;
        }
        if self.blink == Some(true) {
            termion::style::Blink.fmt(f)?;
        }
        if self.invert == Some(true) {
            termion::style::Invert.fmt(f)?;
        }
        if self.strikethrough == Some(true) {
            termion::style::CrossedOut.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    const STYLE_1: Style = Style {
        foreground: Some(Color::Ansi(1)),
        bold: Some(true),
        ..Style::EMPTY
    };

    const STYLE_2: Style = Style {
        foreground: Some(Color::Ansi(2)),
        italic: Some(true),
        ..Style::EMPTY
    };

    #[test]
    fn with() {
        let style = STYLE_1.with(STYLE_2);
        assert_eq!(
            style,
            Style {
                foreground: Some(2.into()),
                bold: Some(true),
                italic: Some(true),
                ..Style::EMPTY
            },
        );
    }

    #[test]
    fn or() {
        let style = STYLE_1.or(STYLE_2);
        assert_eq!(
            style,
            Style {
                foreground: Some(1.into()),
                bold: Some(true),
                italic: Some(true),
                ..Style::EMPTY
            },
        );
    }

    #[test]
    fn plus() {
        assert_eq!(STYLE_1.with(STYLE_2), STYLE_1 + STYLE_2);
    }

    #[test]
    fn print_empty() {
        let mut output = vec![];
        write!(&mut output, "{}", Style::EMPTY).unwrap();
        assert_eq!(output, b"");
    }

    #[test]
    fn print_full() {
        let mut output = vec![];
        write!(
            &mut output,
            "{}",
            Style {
                foreground: Some(1.into()),
                background: Some(2.into()),
                bold: Some(true),
                italic: Some(true),
                underline: Some(true),
                blink: Some(true),
                invert: Some(true),
                strikethrough: Some(true),
            },
        )
        .unwrap();
        assert_eq!(
            output,
            b"\x1b[38;5;1m\x1b[48;5;2m\x1b[1m\x1b[3m\x1b[4m\x1b[5m\x1b[7m\x1b[9m",
        );
    }

    #[test]
    fn print_default_and_rgb() {
        let mut output = vec![];
        write!(
            &mut output,
            "{}",
            Style::fg((1, 2, 3)) + Style::bg(Color::Default),
        )
        .unwrap();
        assert_eq!(output, b"\x1b[38;2;1;2;3m\x1b[49m");
    }
}
