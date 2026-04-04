use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

struct State {
    selected: usize,
}

struct Option {
    name: &'static str,
    icon: char,
    shortcut: char,
}

const OPTIONS: &[Option] = &[
    Option { name: "Open file",   icon: '→', shortcut: 'z' },
    Option { name: "Create file", icon: '→', shortcut: 'b' },
    Option { name: "Help",        icon: '→', shortcut: 's' },
];

pub fn home(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = State { selected: 0 };

    loop {
        terminal.draw(|frame| render(frame, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    state.selected = (state.selected + 1) % OPTIONS.len();
                }
                KeyCode::Up => {
                    if state.selected == 0 {
                        state.selected = OPTIONS.len() - 1;
                    } else {
                        state.selected -= 1;
                    }
                }
                KeyCode::Enter => {
                    // TODO: handle selected option
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, state: &State) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(horizontal[1]);
    frame.render_widget(block, horizontal[1]);

    render_menu(frame, inner, state);
}

fn render_menu(frame: &mut Frame, area: Rect, state: &State) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(area);

    let title = Paragraph::new(
        "
        ██╗  ██╗██╗   ██╗███╗   ██╗ █████╗
        ██║ ██╔╝╚██╗ ██╔╝████╗  ██║██╔══██╗
        █████╔╝  ╚████╔╝ ██╔██╗ ██║███████║
        ██╔═██╗   ╚██╔╝  ██║╚██╗██║██╔══██║
        ██║  ██╗   ██║   ██║ ╚████║██║  ██║
        ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═══╝╚═╝  ╚═╝
        ",
    )
    .alignment(ratatui::layout::HorizontalAlignment::Center)
    .style(Style::default().fg(Color::Blue));

    frame.render_widget(title, layout[0]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(2); OPTIONS.len()])
        .split(layout[1]);

    for (i, option) in OPTIONS.iter().enumerate() {
        let is_selected = state.selected == i;

        let label_style = if is_selected {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        let inner_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(chunks[i]);

        let label = Paragraph::new(format!("{} {}", option.icon, option.name))
            .alignment(ratatui::layout::HorizontalAlignment::Left)
            .style(label_style);

        let shortcut = Paragraph::new(format!("{}", option.shortcut))
            .alignment(ratatui::layout::HorizontalAlignment::Right)
            .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(label, inner_chunks[0]);
        frame.render_widget(shortcut, inner_chunks[1]);
    }
}
