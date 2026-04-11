use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use crate::editor::{
    KynaScene,
    buffer::FileBuffer,
    utils::{get_icon, normalze_mode},
};

pub struct Sidebar {
    buffer: FileBuffer,
}

impl Sidebar {
    pub fn new(buffer: FileBuffer) -> Self {
        Self { buffer }
    }
}

impl KynaScene for Sidebar {
    fn render(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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

        left.render(chunks[0], buf);
        if let Some(branch) = self.buffer.get_breanch() {
            Paragraph::new(format!(" {}", branch))
                .alignment(Alignment::Left)
                .block(Block::default().style(Style::default().bg(Color::Black)))
                .style(Style::default().bg(Color::Black).fg(Color::White))
        } else {
            Paragraph::new("").block(Block::default().style(Style::default().bg(Color::Black)))
        }
        .render(chunks[1], buf);

        line_col.render(chunks[3], buf);
        file_info.render(chunks[2], buf);
    }
}
