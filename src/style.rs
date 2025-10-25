/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use std::fmt;
use std::ops::Add;

use termion::color::{AnsiValue, Bg, Fg};

/// A text style, encompassing the color and other style options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    /// The foreground color.
    pub foreground: Option<u8>,
    /// The background color.
    pub background: Option<u8>,
    /// Whether the text should be bold.
    pub bold: Option<bool>,
    /// Whether the text should be italicized.
    pub italic: Option<bool>,
    /// Whether the text should have its colors inverted.
    pub invert: Option<bool>,
}

impl Style {
    /// The empty style, with nothing specified. Equivalent to `Style::default()`.
    pub const EMPTY: Style = Style {
        foreground: None,
        background: None,
        bold: None,
        italic: None,
        invert: None,
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

    /// Inverted colors.
    pub const INVERT: Style = Style {
        invert: Some(true),
        ..Style::EMPTY
    };

    /// Creates a style with only the foreground specified.
    pub fn fg(value: u8) -> Style {
        Style {
            foreground: Some(value),
            ..Style::EMPTY
        }
    }

    /// Creates a style with only the background specified.
    pub fn bg(value: u8) -> Style {
        Style {
            background: Some(value),
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
            invert: self.invert.or(other.invert),
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

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(foreground) = self.foreground {
            Fg(AnsiValue(foreground)).fmt(f)?;
        }
        if let Some(background) = self.background {
            Bg(AnsiValue(background)).fmt(f)?;
        }
        if self.bold == Some(true) {
            termion::style::Bold.fmt(f)?;
        }
        if self.italic == Some(true) {
            termion::style::Italic.fmt(f)?;
        }
        if self.invert == Some(true) {
            termion::style::Invert.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    const STYLE_1: Style = Style {
        foreground: Some(1),
        bold: Some(true),
        ..Style::EMPTY
    };

    const STYLE_2: Style = Style {
        foreground: Some(2),
        italic: Some(true),
        ..Style::EMPTY
    };

    #[test]
    fn with() {
        let style = STYLE_1.with(STYLE_2);
        assert_eq!(
            style,
            Style {
                foreground: Some(2),
                background: None,
                bold: Some(true),
                italic: Some(true),
                invert: None
            },
        );
    }

    #[test]
    fn or() {
        let style = STYLE_1.or(STYLE_2);
        assert_eq!(
            style,
            Style {
                foreground: Some(1),
                background: None,
                bold: Some(true),
                italic: Some(true),
                invert: None
            },
        );
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
                foreground: Some(1),
                background: Some(2),
                bold: Some(true),
                italic: Some(true),
                invert: Some(true),
            },
        )
        .unwrap();
        assert_eq!(output, b"\x1b[38;5;1m\x1b[48;5;2m\x1b[1m\x1b[3m\x1b[7m");
    }
}
