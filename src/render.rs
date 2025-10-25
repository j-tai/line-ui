/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use std::io::{self, Write};

use termion::style::Reset;
use termion::{clear, cursor};

use crate::Style;
use crate::element::Element;

/// A chunk of text with a constant style to be rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderChunk<'s> {
    /// The content of this chunk.
    pub(crate) value: &'s str,
    /// The width of this chunk.
    pub(crate) width: usize,
    /// The style of this chunk.
    pub(crate) style: Style,
    /// Whether to display the cursor at the start of this chunk. If this is
    /// true, then `value` must be `""`, `width` must be `0`, and `style` must
    /// be `Style::EMPTY`.
    pub(crate) cursor: bool,
}

impl<'s> RenderChunk<'s> {
    pub const CURSOR: RenderChunk<'static> = RenderChunk {
        value: "",
        width: 0,
        style: Style::EMPTY,
        cursor: true,
    };

    pub fn new(value: &'s str, style: Style) -> Self {
        RenderChunk::with_known_width(value, crate::width(value), style)
    }

    pub(crate) fn with_known_width(value: &'s str, width: usize, style: Style) -> Self {
        debug_assert_eq!(crate::width(value), width);
        RenderChunk {
            value,
            width,
            style,
            cursor: false,
        }
    }
}

impl<'s> From<&'s str> for RenderChunk<'s> {
    fn from(value: &'s str) -> Self {
        RenderChunk::new(value, Style::EMPTY)
    }
}

/// A struct that outputs lines to a [writer](Write).
pub struct Renderer<W: Write> {
    pub(crate) writer: W,
    lines_rendered: u16,
    desired_cursor: Option<(u16, u16)>,
    is_dirty: bool, // flag for debugging
}

impl<W: Write> Renderer<W> {
    /// Creates a new [`Renderer`] that writes to the given writer.
    pub fn new(writer: W) -> Self {
        Renderer {
            writer,
            lines_rendered: 0,
            desired_cursor: None,
            is_dirty: false,
        }
    }

    /// Resets the renderer's state.
    fn reset_state(&mut self) {
        self.lines_rendered = 0;
        self.desired_cursor = None;
        self.is_dirty = false;
    }

    /// Resets the cursor position, allowing rendering to start over.
    pub fn reset(&mut self) -> io::Result<&mut Self> {
        assert!(!self.is_dirty, "finalize() must be called after rendering");
        // Reset the cursor to the top-left.
        let current_cursor_line = match self.desired_cursor {
            // If there's a desired cursor position, the cursor is there.
            Some((line, _)) => line,
            // Otherwise, it's the last line rendered.
            None => self.lines_rendered.saturating_sub(1),
        };
        if current_cursor_line != 0 {
            write!(self.writer, "{}", cursor::Up(current_cursor_line))?;
        }
        write!(self.writer, "\r")?;

        self.reset_state();
        Ok(self)
    }

    /// Clears the UI, resetting the terminal back to its initial state.
    ///
    /// Note that this method is automatically called when the `Renderer` is
    /// [dropped](Drop).
    pub fn clear(&mut self) -> io::Result<()> {
        assert!(!self.is_dirty, "finalize() must be called after rendering");
        self.reset()?;
        write!(self.writer, "{}{}", clear::AfterCursor, cursor::Show)
    }

    /// Renders a line.
    pub fn render<E: Element>(&mut self, line: E) -> io::Result<&mut Self> {
        self.is_dirty = true;
        // If this isn't the first line, then move to the next line.
        if self.lines_rendered != 0 {
            write!(self.writer, "\n\r")?;
        }
        // Render each chunk.
        let mut column = 0;
        for chunk in line.render() {
            if chunk.cursor {
                debug_assert_eq!(chunk.value, "");
                debug_assert_eq!(chunk.width, 0);
                self.desired_cursor = Some((self.lines_rendered, column as u16));
            } else {
                write!(self.writer, "{}{}{Reset}", chunk.style, chunk.value)?;
                column += chunk.width;
            }
        }
        write!(self.writer, "{}", clear::UntilNewline)?;
        self.lines_rendered += 1;
        Ok(self)
    }

    /// Finishes rendering. This should be called immediately after the
    /// [`render`](Self::render) calls are complete.
    pub fn finish(&mut self) -> io::Result<()> {
        self.is_dirty = false;
        if let Some((line, column)) = self.desired_cursor {
            let up = self.lines_rendered - line - 1;
            if up != 0 {
                write!(self.writer, "{}", cursor::Up(up))?;
            }
            write!(self.writer, "\r")?;
            if column != 0 {
                write!(self.writer, "{}", cursor::Right(column))?;
            }
            write!(self.writer, "{}", cursor::Show)?;
        } else {
            write!(self.writer, "{}", cursor::Hide)?;
        }
        self.writer.flush()
    }

    /// Leaves the currently-rendered text, making it impossible to clear.
    ///
    /// This method may be used if you want to dispose of this `Renderer`
    /// without clearing the currently-rendered text. This should be called
    /// after [`finish`](Self::finish).
    pub fn leave(&mut self) -> io::Result<()> {
        assert!(!self.is_dirty, "finalize() must be called after rendering");
        if self.lines_rendered == 0 {
            return Ok(());
        }
        let down = match self.desired_cursor {
            Some((row, _)) => self.lines_rendered - row - 1,
            None => 0,
        };
        if down != 0 {
            write!(self.writer, "{}", cursor::Down(down))?;
        }
        write!(self.writer, "\n\r")?;
        self.reset_state();
        Ok(())
    }
}

impl<W: Write> Drop for Renderer<W> {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::element::{Cursor, IntoElement};

    use super::*;

    #[test]
    fn empty() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        for _ in 0..3 {
            r.writer.clear();
            r.reset()?.finish()?;
            assert_eq!(r.writer, b"\r\x1b[?25l");
        }
        Ok(())
    }

    #[test]
    fn empty_line() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        for _ in 0..3 {
            r.writer.clear();
            r.reset()?.render(())?.finish()?;
            assert_eq!(r.writer, b"\r\x1b[K\x1b[?25l");
        }
        Ok(())
    }

    #[test]
    fn one_line() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        for _ in 0..3 {
            r.writer.clear();
            r.reset()?.render("trans rights".into_element())?.finish()?;
            assert_eq!(r.writer, b"\rtrans rights\x1b[m\x1b[K\x1b[?25l");
        }
        Ok(())
    }

    #[test]
    fn two_lines() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render("trans rights".into_element())?
            .render("enby rights".into_element())?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\rtrans rights\x1b[m\x1b[K\n\renby rights\x1b[m\x1b[K\x1b[?25l",
        );

        for _ in 0..3 {
            r.writer.clear();
            r.reset()?
                .render("trans rights".into_element())?
                .render("enby rights".into_element())?
                .finish()?;
            assert_eq!(
                r.writer,
                b"\x1b[1A\rtrans rights\x1b[m\x1b[K\n\renby rights\x1b[m\x1b[K\x1b[?25l",
            );
        }
        Ok(())
    }

    #[test]
    fn drop() {
        let mut out = vec![];
        Renderer::new(&mut out);
        assert_eq!(out, b"\r\x1b[J\x1b[?25h");
    }

    #[test]
    fn cursor_at_start_of_last_line() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render("trans rights".into_element())?
            .render((Cursor, "enby rights".into_element()))?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\rtrans rights\x1b[m\x1b[K\n\renby rights\x1b[m\x1b[K\r\x1b[?25h",
        );
        Ok(())
    }

    #[test]
    fn cursor_in_last_line() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render("trans rights".into_element())?
            .render(("enby ".into_element(), Cursor, "rights".into_element()))?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\rtrans rights\x1b[m\x1b[K\n\renby \x1b[mrights\x1b[m\x1b[K\r\x1b[5C\x1b[?25h",
        );
        Ok(())
    }

    #[test]
    fn cursor_in_previous_line() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render(("trans rights".into_element(), Cursor))?
            .render("enby rights".into_element())?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\rtrans rights\x1b[m\x1b[K\n\renby rights\x1b[m\x1b[K\x1b[1A\r\x1b[12C\x1b[?25h",
        );
        Ok(())
    }

    #[test]
    fn leave_empty() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?.finish()?;
        r.writer.clear();
        r.leave()?;
        assert_eq!(r.writer, b"");
        Ok(())
    }

    #[test]
    fn leave() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render("trans rights".into_element())?
            .render("enby rights".into_element())?
            .finish()?;
        r.writer.clear();
        r.leave()?;
        r.clear()?;
        assert_eq!(r.writer, b"\n\r\r\x1b[J\x1b[?25h");
        Ok(())
    }

    #[test]
    fn leave_with_cursor() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render(("trans rights".into_element(), Cursor))?
            .render("enby rights".into_element())?
            .finish()?;
        r.writer.clear();
        r.leave()?;
        r.clear()?;
        assert_eq!(r.writer, b"\x1b[1B\n\r\r\x1b[J\x1b[?25h");
        Ok(())
    }
}
