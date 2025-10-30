/*
 * Copyright (c) 2025 Jasmine Tai. All rights reserved.
 */

//! An interactive tic-tac-toe game.

use std::io;

use line_ui::element::{Cursor, Element, IntoElement};
use line_ui::{Renderer, Style};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug, Clone, Copy, Default)]
enum Player {
    #[default]
    X,
    O,
}

#[derive(Debug, Default)]
struct TicTacToe {
    pub grid: [[Option<Player>; 3]; 3],
    pub current: Player,
}

impl TicTacToe {
    fn place(&mut self, row: usize, col: usize) {
        if self.grid[row][col].is_some() {
            // Square already occupied
            return;
        }
        self.grid[row][col] = Some(self.current);
        self.current = match self.current {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }

    fn check_win(&self) -> Option<Option<Player>> {
        for i in 0..3 {
            for line in [
                self.grid[i],                           // row
                [0, 1, 2].map(|j| self.grid[j][i]),     // column
                [0, 1, 2].map(|j| self.grid[j][j]),     // diagonal
                [0, 1, 2].map(|j| self.grid[2 - j][j]), // other diagonal
            ] {
                match line {
                    [Some(Player::X), Some(Player::X), Some(Player::X)] => {
                        return Some(Some(Player::X));
                    }
                    [Some(Player::O), Some(Player::O), Some(Player::O)] => {
                        return Some(Some(Player::O));
                    }
                    _ => {}
                }
            }
        }

        if self.grid.as_flattened().iter().all(Option::is_some) {
            Some(None) // draw; all cells filled
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let mut r = Renderer::new(stdout);
    let mut events = std::io::stdin().events();

    let mut game = TicTacToe::default();
    let (mut row, mut col): (usize, usize) = (1, 1);
    loop {
        // Render the grid
        r.reset()?;
        for (i, line) in game.grid.iter().enumerate() {
            if i != 0 {
                r.render("--+---+--".into_element())?;
            }
            r.render((
                ((row, col) == (i, 0)).then_some(Cursor),
                render_player(line[0]),
                " | ".into_element(),
                ((row, col) == (i, 1)).then_some(Cursor),
                render_player(line[1]),
                " | ".into_element(),
                ((row, col) == (i, 2)).then_some(Cursor),
                render_player(line[2]),
            ))?;
        }

        // Check if someone won
        let result = game.check_win();
        match result {
            Some(Some(winner)) => {
                r.render((
                    "The winner is ".into_element(),
                    render_player(Some(winner)),
                    "!".into_element(),
                ))?;
            }
            Some(None) => {
                r.render("The game is a draw.".into_element())?;
            }
            _ => {}
        }

        r.finish()?;
        if result.is_some() {
            r.leave()?;
            break;
        }

        // Poll an input
        let Some(event) = events.next() else {
            break;
        };
        match event? {
            Event::Key(Key::Up) => row = row.checked_sub(1).unwrap_or(2),
            Event::Key(Key::Down) => row = (row + 1) % 3,
            Event::Key(Key::Left) => col = col.checked_sub(1).unwrap_or(2),
            Event::Key(Key::Right) => col = (col + 1) % 3,
            Event::Key(Key::Char(' ' | '\n' | '\r')) => game.place(row, col),
            _ => {}
        }
    }

    Ok(())
}

fn render_player(player: Option<Player>) -> impl Element<'static> {
    match player {
        None => "-".with_style(Style::fg(245)),
        Some(Player::X) => "X".with_style(Style::BOLD + Style::fg(33)),
        Some(Player::O) => "O".with_style(Style::BOLD + Style::fg(203)),
    }
}
