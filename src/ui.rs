use ratatui::layout::Layout;
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};

#[derive(Default)]
struct Ui {
    board: Rect,
}

impl Ui {
    fn init(area: Rect) {
        let layout = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);

        let columns = layout.split(area);

        let zxc: Vec<Rect> = vec![];
    }
}
