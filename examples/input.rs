/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

use line_ui::element::{Cursor, Direction, Gap, IntoElement};
use line_ui::{Renderer, Style};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let mut r = Renderer::new(stdout);

    let mut name = String::new();

    let mut events = std::io::stdin().events();
    loop {
        r.clear()?;
        r.render((
            "Enter your name: ".into_element(),
            (name.into_element(), Cursor, Gap(1))
                .fixed_width(20)
                .truncated(Direction::Left)
                .with_style(Style::bg(240)),
        ))?;
        r.finish()?;

        let Some(event) = events.next().transpose()? else {
            break;
        };
        match event {
            Event::Key(Key::Char(ch)) if !ch.is_ascii_control() => name.push(ch),
            Event::Key(Key::Char('\n' | '\r')) => break,
            Event::Key(Key::Backspace) => {
                name.pop();
            }
            _ => {}
        }
    }

    r.clear()?;
    drop(r);
    println!("Your name is {name:?}");
    Ok(())
}
