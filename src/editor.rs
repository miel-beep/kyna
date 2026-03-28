use buffer;
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
use std::fs::File;

struct Buffer {
    text_file: String,

}

pub fn start() -> std::io::Result<()> {
    let mut string_file: String = std::fs::read_to_string("cudecachorro.txt").expect("erro");
    let mut buffer = Buffer { 
         text_file: string_file
    };

    loop {
        let backend = CrosstermBackend::new(stderr());
        let mut terminal = Terminal::new(backend)?;

        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        terminal.clear()?;
        terminal.draw(|frame| render(&buffer, frame))?;
        if let Event::Key(key) = event::read()? { 
           match key.code {
               KeyCode::Char('q') => {
                    std::fs::write("cudecachorro.txt", buffer.text_file).unwrap();
                    stdout().execute(LeaveAlternateScreen)?;
                    disable_raw_mode()?;
                    break;
                },
                KeyCode::Char(c) => buffer.text_file.push(c),
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
