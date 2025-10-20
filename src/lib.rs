#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod element;
mod render;
mod style;

pub use render::Renderer;
pub use style::*;

#[cfg(test)]
mod tests {
    use std::io;

    use crate::element::IntoElement;

    use super::*;

    #[test]
    fn with_style() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render((
                "one".into_element(),
                "two".with_style(Style::fg(4).with(Style::bg(5))),
                "three".into_element(),
            ))?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\rone\x1b[m\x1b[38;5;4m\x1b[48;5;5mtwo\x1b[mthree\x1b[m\x1b[?25l",
        );
        Ok(())
    }

    #[test]
    fn with_styled_fixed_width() -> io::Result<()> {
        let mut r = Renderer::new(vec![]);
        r.reset()?
            .render(
                "test"
                    .fixed_width(10)
                    .with_style(Style::fg(42).with(Style::bg(43))),
            )?
            .finish()?;
        assert_eq!(
            r.writer,
            b"\r\x1b[38;5;42m\x1b[48;5;43mtest\x1b[m\x1b[38;5;42m\x1b[48;5;43m      \x1b[m\x1b[?25l",
        );
        Ok(())
    }
}
