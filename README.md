# line-ui

Line-based TUI library.

## Example

```rust+no_test
use line_ui::{Renderer, Style};
use line_ui::element::*;
use termion::raw::IntoRawMode;

fn main() -> std::io::Result<()> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let mut r = Renderer::new(stdout);

    r.clear()?;
    r.render((
        "left".into_element(),
        "fixed width".with_style(Style::INVERT).fixed_width(20),
        "right".into_element(),
    ))?;
    r.finish()?;
    Ok(())
}
```
