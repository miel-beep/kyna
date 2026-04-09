use ratatui::layout::{Alignment, Layout};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};

use crate::editor::buffer::Buffer;
use crate::editor::utils::{get_icon, normalze_mode};

pub struct Ui<'a> {
    buffer: &'a Buffer,
}

impl<'a> Ui<'a> {
    pub fn new(buffer: &'a Buffer) -> Self {
        Self { buffer }
    }
    pub fn side_bar(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(0),
                Constraint::Min(0),
                Constraint::Length(13),
            ])
            .split(area);

        let (icon, color) = get_icon(self.buffer.name.to_str().unwrap_or(""));
        let line = Line::from(vec![
            Span::styled(icon, Style::default().fg(color).bg(Color::Black)),
            Span::styled(
                format!(" {}", self.buffer.name.to_str().unwrap_or("")),
                Style::default().bg(Color::Black).fg(Color::White),
            ),
        ]);
        let file_info = Paragraph::new(line)
            .block(Block::default().style(Style::default().bg(Color::Black)))
            .alignment(Alignment::Right);

        let left = Paragraph::new(normalze_mode(self.buffer.mode.clone()))
            .style(Style::default().bg(Color::Black).fg(Color::White))
            .block(Block::default().style(Style::default().bg(Color::Black)));

        let line_col = Paragraph::new(format!(
            " Ln {}, Col {}",
            self.buffer.cursor.x, self.buffer.cursor.y
        ))
        .block(Block::default().style(Style::default().bg(Color::Black)))
        .style(Style::default().bg(Color::Black).fg(Color::White));

        let branch = self.buffer.get_breanch().map(|b| {
            Paragraph::new(format!(" {}", b))
                .alignment(Alignment::Left)
                .block(Block::default().style(Style::default().bg(Color::Black)))
                .style(Style::default().bg(Color::Black).fg(Color::White))
        });

        frame.render_widget(left, chunks[0]);


        if let Some(branch) = &branch {
            frame.render_widget(branch.clone(), chunks[1]);
        } else {
            let empty =
                Paragraph::new("").block(Block::default().style(Style::default().bg(Color::Black)));
            frame.render_widget(empty, chunks[1]);
        }

        frame.render_widget(line_col, chunks[3]);
        frame.render_widget(file_info, chunks[2]);
    }

}
