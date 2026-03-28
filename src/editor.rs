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

pub fn start() -> std::io::Result<()> {
    loop {
        let backend = CrosstermBackend::new(stderr());
        let mut terminal = Terminal::new(backend)?;

        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        terminal.clear()?;
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? { 
           match key.code {
               KeyCode::Char('q') => {
                    stdout().execute(LeaveAlternateScreen)?;
                    disable_raw_mode()?;
                    break;
                },
                _ => {}
            }
        }
    }
   Ok(()) 
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [first, second] = main.layout(&horizontal);

    let read_file: String = std::fs::read_to_string("cudecachorro.txt").expect("erro");
    
    let paragraph = Paragraph::new(read_file)
        .style(Color::White)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, frame.area());
}
