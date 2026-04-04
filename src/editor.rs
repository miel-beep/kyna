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
    text_file: Vec<String>,

    cursor_pos: Cursor,
}
fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len()) // Se não achar, assume o final
}
impl Buffer {
    fn new(name_file: PathBuf, text_file: Vec<String>) -> Self {
        Self {
            name_file,
            text_file,
            cursor_pos: Cursor {
                cursor_x: 0,
                cursor_y: 0,
            },
        }
    }
    fn len(&self) -> usize {
        self.text_file.len()
    }

    fn insert_char(&mut self, c: char) {
        let y = self.cursor_pos.cursor_y as usize;
        let x = self.cursor_pos.cursor_x as usize;

        let byte_idx = char_to_byte_idx(&self.text_file[y], x);
        self.text_file[y].insert(byte_idx, c);
        self.cursor_pos.cursor_x += 1;
    }

    fn hander_enter(&mut self) {
        let x = self.cursor_pos.cursor_x as usize;
        let y = self.cursor_pos.cursor_y as usize;
        let current_line = &self.text_file[y];

        let before_cursor = current_line[..x].to_string();
        let after_cursor = current_line[x..].to_string();

        self.text_file[y] = before_cursor;
        self.text_file.insert(y + 1, after_cursor);

        self.cursor_pos.cursor_y += 1;
        self.cursor_pos.cursor_x = 0;
    }
    fn handle_backspace(&mut self) {
        let x = self.cursor_pos.cursor_x as usize;
        let y = self.cursor_pos.cursor_y as usize;

        if x > 0 {
            self.cursor_pos.cursor_x -= 1;

            let byte_idx = char_to_byte_idx(&self.text_file[y], self.cursor_pos.cursor_x as usize);
            self.text_file[y].remove(byte_idx);
        } else if y > 0 {
            let current_line_content = self.text_file.remove(y);
            self.cursor_pos.cursor_y -= 1;
            let prev_y = self.cursor_pos.cursor_y as usize;
            let old_len = self.text_file[prev_y].len() as u16;
            self.cursor_pos.cursor_x = old_len;
            self.text_file[prev_y].push_str(&current_line_content);
        }
    }
}
struct Cursor {
    cursor_x: u16,
    cursor_y: u16,
}
fn pasing_file(string_file: &mut String) -> Vec<&str> {
    string_file.split("\n").collect()
}
pub fn start(file: String) -> std::io::Result<()> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let dir = PathBuf::from(file);

    if !dir.exists(){
        File::create(&dir)?;
    }

    let mut string_file: String = std::fs::read_to_string(&dir).expect("erro");
    let text_file = pasing_file(&mut string_file);
    let file = text_file.into_iter().map(|e| e.to_string()).collect();
    let mut buffer = Buffer {
        name_file: dir,
        text_file: file,
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
                    let content = buffer.text_file.join("\n");
                    std::fs::write(buffer.name_file, content).unwrap();

                    stdout().execute(LeaveAlternateScreen)?;
                    disable_raw_mode()?;
                    break;
                }
                KeyCode::Char(c) => {
                    buffer.insert_char(c);
                }

                KeyCode::Down => {
                    if (buffer.cursor_pos.cursor_y as usize) < buffer.len() - 1 {
                        buffer.cursor_pos.cursor_y += 1;
                        let new_line_len =
                            buffer.text_file[buffer.cursor_pos.cursor_y as usize].len() as u16;
                        if buffer.cursor_pos.cursor_x > new_line_len {
                            buffer.cursor_pos.cursor_x = new_line_len;
                        }
                    }
                }
                KeyCode::Backspace => {
                    buffer.handle_backspace();
                }
                KeyCode::Enter => {
                    buffer.hander_enter();
                }
                KeyCode::Up => {
                    if buffer.cursor_pos.cursor_y > 0 {
                        buffer.cursor_pos.cursor_y -= 1
                    }
                }
                KeyCode::Right => {
                    if (buffer.cursor_pos.cursor_x as usize)
                        < buffer.text_file[buffer.cursor_pos.cursor_y as usize].len()
                    {
                        buffer.cursor_pos.cursor_x += 1
                    } else {
                        buffer.cursor_pos.cursor_x = 0;
                        if (buffer.cursor_pos.cursor_y as usize) < buffer.len() - 1 {
                            buffer.cursor_pos.cursor_y += 1;
                        }
                    }
                }
                KeyCode::Left => {
                    if buffer.cursor_pos.cursor_x > 0 {
                        buffer.cursor_pos.cursor_x -= 1
                    }
                    if buffer.cursor_pos.cursor_x == 0 {
                        if (buffer.cursor_pos.cursor_y as usize) > 0 {
                            buffer.cursor_pos.cursor_y -= 1;
                        }
                    }
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
    let display = arg.text_file.join("\n");
    let paragraph = Paragraph::new(display)
        .style(Color::White)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, frame.area());
}
