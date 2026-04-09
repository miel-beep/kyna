mod buffer;
mod render;
mod ui;
mod utils;

use std::fs::File;
use std::io::stdout;
use std::path::PathBuf;

use buffer::Buffer;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

#[derive(Debug, PartialEq, Clone)]
pub enum Modes {
    Normal,
    Insert,
}

pub fn start(file: String) -> std::io::Result<()> {
    let path = PathBuf::from(&file);

    if !path.exists() {
        File::create(&path)?;
    }

    let content = std::fs::read_to_string(&path)?;
    let lines: Vec<String> = content.lines().map(str::to_string).collect();
    let lines = if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    };

    let mut buffer = Buffer::new(path, lines);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(|frame| render::render(&buffer, frame, frame.area()))?;
    terminal.show_cursor()?;

    loop {
        terminal.draw(|frame| render::render(&buffer, frame, frame.area()))?;
        terminal.show_cursor()?;

        if let Event::Key(key) = event::read()? {
            match buffer.mode {
                Modes::Insert => match key.code {
                    KeyCode::Char(c) => buffer.insert_char(c),
                    KeyCode::Enter => buffer.handle_enter(),
                    KeyCode::Backspace => buffer.handle_backspace(),
                    KeyCode::Up => buffer.move_up(),
                    KeyCode::Down => buffer.move_down(),
                    KeyCode::Left => buffer.move_left(),
                    KeyCode::Right => buffer.move_right(),
                    KeyCode::Esc => buffer.mode = Modes::Normal,
                    _ => {}
                },
                Modes::Normal => match key.code {
                    KeyCode::Char('i') => buffer.mode = Modes::Insert,
                    KeyCode::Up => buffer.move_up(),
                    KeyCode::Down => buffer.move_down(),
                    KeyCode::Left => buffer.move_left(),
                    KeyCode::Right => buffer.move_right(),
                    KeyCode::Char('q') => {
                        buffer.save()?;
                        stdout().execute(LeaveAlternateScreen)?;
                        disable_raw_mode()?;
                        break;
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
