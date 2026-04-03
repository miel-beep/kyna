use std::io::{stderr, stdout};

use crossterm::ExecutableCommand;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Position;
use ratatui::style::{Color, Modifier, Style};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{prelude::*, widgets::*};
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use std::fs::File;
use std::path::PathBuf;

struct Buffer {
    name_file: PathBuf,
    text_file: String,
    cursor_pos: Cursor,
}
impl Buffer {
    fn len(&self) -> usize {
        self.text_file
            .lines()
            .nth(self.cursor_pos.cursor_y as usize)
            .map(|line| line.len())
            .unwrap_or(0)
    }
    fn all_lines(&self) -> usize {
        self.text_file.lines().count()
    }
}
struct Cursor {
    cursor_x: u16,
    cursor_y: u16,
}

pub fn start(file: String) -> std::io::Result<()> {
    let mut backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut dir = PathBuf::from(file);
    let mut string_file: String = std::fs::read_to_string(&dir).expect("erro");
    let mut buffer = Buffer {
        name_file: dir,
        text_file: string_file,
        cursor_pos: Cursor {
            cursor_x: 0,
            cursor_y: 0,
        },
    };

    loop {
        terminal.clear()?;
        terminal.draw(|frame| render(&buffer, frame))?;
        terminal.show_cursor()?;

        terminal.set_cursor_position(Position {
            x: buffer.cursor_pos.cursor_x,
            y: buffer.cursor_pos.cursor_y,
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    std::fs::write(buffer.name_file, buffer.text_file).unwrap();
                    stdout().execute(LeaveAlternateScreen)?;
                    disable_raw_mode()?;
                    break;
                }
                KeyCode::Char(c) => buffer.text_file.push(c),

                KeyCode::Down => {
                    if (buffer.cursor_pos.cursor_y as usize) < buffer.all_lines() - 1 {
                        buffer.cursor_pos.cursor_y += 1;
                        let new_line_len = buffer.len() as u16;
                        if buffer.cursor_pos.cursor_x > new_line_len {
                            buffer.cursor_pos.cursor_x = new_line_len;
                        }
                    }
                }
                KeyCode::Up => {
                    if buffer.cursor_pos.cursor_y > 0 {
                        buffer.cursor_pos.cursor_y -= 1
                    }
                }
                KeyCode::Right => {
                    if (buffer.cursor_pos.cursor_x as usize) < buffer.len() {
                        buffer.cursor_pos.cursor_x += 1
                    } else {
                        buffer.cursor_pos.cursor_x = 0;
                        if (buffer.cursor_pos.cursor_y as usize) < buffer.all_lines() - 1 {
                            buffer.cursor_pos.cursor_y += 1;
                        }
                    }
                }
                KeyCode::Left => {
                    if buffer.cursor_pos.cursor_x > 0 {
                        buffer.cursor_pos.cursor_x -= 1
                    }
                }
                KeyCode::Enter => {
                    buffer.text_file.insert(buffer.cursor_pos.cursor_x as usize, '\n');
                    buffer.cursor_pos.cursor_x = 0;
                    buffer.cursor_pos.cursor_y += 1;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(arg: &Buffer, frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [first, second] = main.layout(&horizontal);

    let paragraph = Paragraph::new(arg.text_file.clone())
        .style(Color::White)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, frame.area());
}
