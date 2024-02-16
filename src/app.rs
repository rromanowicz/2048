#![allow(unused)]

use std::{io, usize};

use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{random, Fill, Rng};
use ratatui::layout::Layout;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

use crate::board::{self, Board, Direction};

pub struct App {
    board: Board,
    size: u16,
}

impl App {
    pub fn new<'a>(s: u16) -> App {
        return App {
            board: Board::new(s),
            size: s,
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
                        Char('l') | Right => self.board.shift(Direction::RIGHT),
                        Char('h') | Left => self.board.shift(Direction::LEFT),
                        Char('k') | Up => self.board.shift(Direction::UP),
                        Char('j') | Down => self.board.shift(Direction::DOWN),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        let square_size = self.size * 10;
        let h_borders = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(square_size),
            Constraint::Fill(1),
        ]);
        let v_borders = Layout::vertical([Constraint::Length(square_size / 2)]);
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
        let h_layout = Layout::horizontal(
            (0..self.size)
                .map(|it| Constraint::Percentage(100 / self.size))
                .collect::<Vec<Constraint>>(),
        );
        let v_layout = Layout::vertical(
            (0..self.size)
                .map(|it| Constraint::Percentage(100 / self.size))
                .collect::<Vec<Constraint>>(),
        );

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
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .fg(get_color(value))
                            .padding(Padding::new(0, 0, row_area[j].height / 2 - 1, 0)),
                    )
                    .centered()
                    .render(row_area[j], buf)
            }
        }
    }
}

fn get_color(i: i16) -> Color {
    match i {
        2 => Color::Cyan,
        4 => Color::Blue,
        8 => Color::LightYellow,
        16 => Color::Yellow,
        32 => Color::LightRed,
        64 => Color::Red,
        128 => Color::LightGreen,
        256 => Color::Green,
        512 => Color::LightMagenta,
        1024 => Color::Magenta,
        2048 => Color::Black,
        _ => Color::White,
    }
}
