use std::io::{stderr, stdout};

use crossterm::ExecutableCommand;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode
};
use crossterm::event;
use crossterm::event::{KeyEvent, Event, KeyCode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::{prelude::*, widgets::{Paragraph, Block, Wrap}};
use ratatui::style::{Style, Color, Modifier};
use ratatui::{prelude::*, widgets::*};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::Position;
use std::fs::File;
use std::path::PathBuf;
use ratatui::backend::Backend;

struct Buffer {
    name_file: PathBuf,
    text_file: String,
    cursor_pos: Cursor
}

struct Cursor {
    cursor_x: u16,
    cursor_y: u16
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
             cursor_y: 0
         }
    };


    loop { 
        terminal.clear()?;
        terminal.draw(|frame| render(&buffer, frame))?;
        terminal.show_cursor()?;
        terminal.set_cursor_position(Position {
            x: buffer.cursor_pos.cursor_x,
            y: buffer.cursor_pos.cursor_y
        })?;
        if let Event::Key(key) = event::read()? { 
           match key.code {
               KeyCode::Char('q') => {
                    std::fs::write(buffer.name_file, buffer.text_file).unwrap();
                    stdout().execute(LeaveAlternateScreen)?;
                    disable_raw_mode()?;
                    break;
                },
                KeyCode::Char(c) => buffer.text_file.push(c),
                KeyCode::Down => buffer.cursor_pos.cursor_y += 1, 
                KeyCode::Up => if buffer.cursor_pos.cursor_y > 0 {
                    buffer.cursor_pos.cursor_y -= 1
                },
                KeyCode::Right => buffer.cursor_pos.cursor_x += 1,
                KeyCode::Left => if buffer.cursor_pos.cursor_x > 0 {
                    buffer.cursor_pos.cursor_x -= 1
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
