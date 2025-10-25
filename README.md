# line-ui

Line-based TUI library, using `termion`.

## Example

```rust,no_run
use line_ui::{Renderer, Style};
use line_ui::element::*;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let mut r = Renderer::new(stdout);

    r.reset()?
        .render((
            "left".into_element(),
            "fixed width".fixed_width(20).with_style(Style::INVERT),
            "right".into_element(),
        ))?
        .render("Press any key to continue!".into_element())?
        .finish()?;

    let _ = std::io::stdin().events().next();
    Ok(())
}
```

## License

MIT
