#![allow(unused)]

use std::{io, usize};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{random, Fill, Rng};
use ratatui::layout::Layout;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

use crate::board::{self, Board};

pub struct App {
    board: Board,
}

impl App {
    pub fn new<'a>() -> App {
        return App {
            board: Board::new(),
        };
    }
}

impl App {
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        loop {
            self.draw(&mut terminal)?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') => return Ok(()),
                        Char('l') | Right => self.board.shift_right(),
                        Char('h') | Left => self.board.shift_left(),
                        Char('k') | Up => self.board.shift_up(),
                        Char('j') | Down => self.board.shift_down(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        let square_size = 40;
        let h_borders = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(square_size),
            Constraint::Fill(1),
        ]);
        let v_borders = Layout::vertical([Constraint::Percentage(square_size)]);
        let [left, h_middle, right] = h_borders.areas(terminal.size()?);
        let [v_middle] = v_borders.areas(h_middle);

        terminal.draw(|f| {
            f.render_widget(self, v_middle);
        })?;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let h_layout = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);
        let v_layout = Layout::vertical([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

        let rows = v_layout.split(area);

        for i in 0..self.board.items.len() {
            let row_area = h_layout.split(rows[i]);
            for j in 0..self.board.items[i].len() {
                let value = self.board.items[i][j];
                let text = if value == 0 {
                    String::from("")
                } else {
                    value.to_string()
                };
                Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).padding(Padding::new(
                        0,
                        0,
                        row_area[j].height / 2 - 1,
                        0,
                    )))
                    .centered()
                    .render(row_area[j], buf)
            }
        }
    }
}
