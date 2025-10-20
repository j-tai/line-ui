/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use std::io::{self, Write};

use termion::style::Reset;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthStr;

use crate::Style;
use crate::element::Element;

/// A chunk of text with a constant style to be rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderChunk<'s> {
    pub(crate) value: &'s str,
    pub(crate) width: usize,
    pub(crate) style: Style,
    pub(crate) cursor: Option<usize>,
}

impl<'s> RenderChunk<'s> {
    pub fn new(value: &'s str, style: Style) -> Self {
        RenderChunk::with_cursor(value, style, None)
    }

    pub fn with_cursor(value: &'s str, style: Style, cursor: impl Into<Option<usize>>) -> Self {
        RenderChunk {
            value,
            width: value.width(),
            style,
            cursor: cursor.into(),
        }
    }
}

impl<'s> From<&'s str> for RenderChunk<'s> {
    fn from(value: &'s str) -> Self {
        RenderChunk::new(value, Style::EMPTY)
    }
}

/// A struct that outputs lines to a [writer](Write).
pub struct Renderer<W> {
    writer: W,
    lines_rendered: u16,
    desired_cursor: Option<(u16, u16)>,
}

impl<W: Write> Renderer<W> {
    /// Creates a new [`Renderer`] that writes to the given writer.
    pub fn new(writer: W) -> Self {
        Renderer {
            writer,
            lines_rendered: 0,
            desired_cursor: None,
        }
    }

    /// Destroys this renderer and returns the original writer.
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Clears the rendering area. This should be called before
    /// [`render`](Self::render).
    pub fn clear(&mut self) -> io::Result<()> {
        // Start by resetting the cursor to the top-left.
        let current_cursor_line = match self.desired_cursor {
            // If there's a desired cursor position, the cursor is there.
            Some((line, _)) => line,
            // Otherwise, it's the last line rendered.
            None => self.lines_rendered.saturating_sub(1),
        };
        if current_cursor_line != 0 {
            write!(self.writer, "{}", cursor::Up(current_cursor_line))?;
        }

        // Now clear everything after the cursor.
        write!(self.writer, "\r{}", clear::AfterCursor)?;

        // Finally, reset the state.
        self.lines_rendered = 0;
        self.desired_cursor = None;
        Ok(())
    }

    /// Renders a line.
    pub fn render<E: Element>(&mut self, line: E) -> io::Result<()> {
        // If this isn't the first line, then move to the next line.
        if self.lines_rendered != 0 {
            write!(self.writer, "\n\r")?;
        }
        // Render each chunk.
        let mut column = 0;
        for chunk in line.render() {
            write!(self.writer, "{}{}{Reset}", chunk.style, chunk.value)?;

            if let Some(cursor) = chunk.cursor {
                self.desired_cursor = Some((self.lines_rendered, (column + cursor) as u16));
            }
            column += chunk.width;
        }
        self.lines_rendered += 1;
        Ok(())
    }

    /// Finishes rendering. This should be called after [`render`](Self::render)
    /// and before polling inputs.
    pub fn finish(&mut self) -> io::Result<()> {
        if let Some((line, column)) = self.desired_cursor {
            let up = self.lines_rendered - line - 1;
            if up != 0 {
                write!(self.writer, "{}", cursor::Up(up))?;
            }
            write!(self.writer, "\r")?;
            if column != 0 {
                write!(self.writer, "{}", cursor::Right(column))?;
            }
        } else {
            write!(self.writer, "{}", cursor::Hide)?;
        }
        self.writer.flush()
    }
}

impl<W> std::ops::Deref for Renderer<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl<W> std::ops::DerefMut for Renderer<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

#[cfg(test)]
mod tests {
    use crate::element::IntoElement;

    use super::*;

    #[test]
    fn empty() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.finish().unwrap();
        assert_eq!(out, b"\r\x1b[J\x1b[?25l");
    }

    #[test]
    fn empty_reuse() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.finish().unwrap();
        r.writer.clear();

        r.clear().unwrap();
        r.finish().unwrap();
        assert_eq!(out, b"\r\x1b[J\x1b[?25l");
    }

    #[test]
    fn one_line() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.render("trans rights".into_element()).unwrap();
        r.finish().unwrap();
        assert_eq!(out, b"\r\x1b[Jtrans rights\x1b[m\x1b[?25l");
    }

    #[test]
    fn one_line_reuse() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.render("one".into_element()).unwrap();
        r.finish().unwrap();
        r.writer.clear();

        r.clear().unwrap();
        r.render("trans rights".into_element()).unwrap();
        r.finish().unwrap();
        assert_eq!(out, b"\r\x1b[Jtrans rights\x1b[m\x1b[?25l");
    }

    #[test]
    fn two_lines() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.render("trans rights".into_element()).unwrap();
        r.render("enby rights".into_element()).unwrap();
        r.finish().unwrap();
        assert_eq!(
            out,
            b"\r\x1b[Jtrans rights\x1b[m\n\renby rights\x1b[m\x1b[?25l",
        );
    }

    #[test]
    fn two_lines_reuse() {
        let mut out = vec![];
        let mut r = Renderer::new(&mut out);
        r.clear().unwrap();
        r.render("trans rights".into_element()).unwrap();
        r.render("enby rights".into_element()).unwrap();
        r.finish().unwrap();
        r.writer.clear();

        r.clear().unwrap();
        r.render("trans rights".into_element()).unwrap();
        r.render("enby rights".into_element()).unwrap();
        r.finish().unwrap();
        assert_eq!(
            out,
            b"\x1b[1A\r\x1b[Jtrans rights\x1b[m\n\renby rights\x1b[m\x1b[?25l",
        );
    }
}
