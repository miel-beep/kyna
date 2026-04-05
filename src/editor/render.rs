use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Rect, Position},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::editor::buffer::Buffer;

pub fn render(buffer: &Buffer, frame: &mut Frame, area: Rect) {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(5), Constraint::Min(1)])
        .split(area);
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
    let editor = Paragraph::new(content)
        .style(Style::default().fg(Color::White));
    frame.render_widget(gutter, horizontal[0]);
    frame.render_widget(editor, horizontal[1]);


    frame.set_cursor_position(Position {
        x: horizontal[1].x + buffer.cursor.x,
        y: horizontal[1].y + buffer.cursor.y,
    });
}
