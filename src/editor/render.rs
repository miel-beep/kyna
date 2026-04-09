use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::editor::{buffer::Buffer, ui::Ui};

pub fn render(buffer: &Buffer, frame: &mut Frame, area: Rect) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(vertical[0]);
    let numbers: String = buffer
        .lines
        .iter()
        .enumerate()
        .map(|(i, _)| format!("{:>3} \n", i + 1))
        .collect();
    let gutter = Paragraph::new(numbers)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default());
    let content = buffer.lines.join("\n");
    let editor = Paragraph::new(content).style(Style::default().fg(Color::White));
    frame.render_widget(gutter, horizontal[0]);
    frame.render_widget(editor, horizontal[1]);

    let ui = Ui::new(buffer);
    ui.side_bar(frame, vertical[1]);
    frame.set_cursor_position((
        horizontal[0].x + 5 + buffer.cursor.x,
        horizontal[1].y + buffer.cursor.y,
    ));
}
