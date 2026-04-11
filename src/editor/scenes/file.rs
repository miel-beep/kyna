use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

use crate::editor::{KynaCommand, KynaScene, Modes, buffer::FileBuffer};

pub struct FileScene {
    buffer: FileBuffer,
}

impl KynaScene for FileScene {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> KynaCommand {
        match self.buffer.mode {
            Modes::Insert => match key.code {
                KeyCode::Char(c) => self.buffer.insert_char(c),
                KeyCode::Enter => self.buffer.handle_enter(),
                KeyCode::Backspace => self.buffer.handle_backspace(),
                KeyCode::Up => self.buffer.move_up(),
                KeyCode::Down => self.buffer.move_down(),
                KeyCode::Left => self.buffer.move_left(),
                KeyCode::Right => self.buffer.move_right(),
                KeyCode::Esc => self.buffer.mode = Modes::Normal,
                _ => {}
            },
            Modes::Normal => match key.code {
                KeyCode::Char('i') => self.buffer.mode = Modes::Insert,
                KeyCode::Up => self.buffer.move_up(),
                KeyCode::Down => self.buffer.move_down(),
                KeyCode::Left => self.buffer.move_left(),
                KeyCode::Right => self.buffer.move_right(),
                KeyCode::Char('q') => {
                    self.buffer.save();
                    return KynaCommand::Exit;
                }
                _ => {}
            },
            Modes::Select => {}
        };
        KynaCommand::None
    }
}

impl Widget for FileScene {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
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
        let numbers: String = self
            .buffer
            .lines
            .iter()
            .enumerate()
            .map(|(i, _)| format!("{:>3} \n", i + 1))
            .collect();
        let gutter = Paragraph::new(numbers)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default());
        let content = self.buffer.lines.join("\n");
        let editor = Paragraph::new(content).style(Style::default().fg(Color::White));
        gutter.render(horizontal[0], buf);
        editor.render(horizontal[1], buf);
    }
}
