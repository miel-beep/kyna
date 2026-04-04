
use std::rc::Rc;

use ratatui::{layout::{
    Constraint, Direction, Layout, Rect

}, style::{Color, Style}, widgets::{Block, Borders, Padding, Paragraph}};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal};
struct State {
    y: i32,
}
pub fn home2(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state = State {  y: 0 };
    loop {
        terminal.draw(|frame| {
            redner(frame);
        });
        if let Event::Key(key) = event::read()?  {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    if state.y > 0{
                        state.y-=1
                    }else if state.y  == 0{
                        state.y = 3;
                    }
                },
                KeyCode::Up => {

                    if  state.y < 3{
                        state.y+=1;
                    }else if state.y ==3{
                        state.y =0;
                    }
                }

                _ => {}
            }
        }
    }
    Ok(())
}
struct BlockUI{
    vertical: Rc<[Rect]>,
    horizontal: Rc<[Rect]>,

}
struct OptionUI {
    name: String,
    icon: char,

    atalho: char
}

fn render_options(frame: &mut ratatui::Frame, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // altura do título (ajusta aqui)
            Constraint::Min(0),    // resto = opções
        ])
        .split(area);
    let title_area = layout[0];
    let options_area = layout[1];
    let title = Paragraph::new("
        ██╗  ██╗██╗   ██╗███╗   ██╗ █████╗
        ██║ ██╔╝╚██╗ ██╔╝████╗  ██║██╔══██╗
        █████╔╝  ╚████╔╝ ██╔██╗ ██║███████║
        ██╔═██╗   ╚██╔╝  ██║╚██╗██║██╔══██║
        ██║  ██╗   ██║   ██║ ╚████║██║  ██║
        ╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═══╝╚═╝  ╚═╝

        ").alignment(ratatui::layout::HorizontalAlignment::Center)
    .style(Style::default().fg(Color::Blue));
    frame.render_widget(title, title_area);

    let options = [
        OptionUI { name: "Open file".into(), icon: '←',  atalho: 'z' },
        OptionUI { name: "Create file".into(), icon: '←', atalho: 'b'},
        OptionUI { name: "Help".into(), icon: '←',  atalho: 's'},

    ];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(2); options.len()])
        .split(options_area);

    for (i, option) in options.iter().enumerate() {
        let style = Style::default().fg(Color::Blue);

        let widget = Paragraph::new(format!("{} {}", option.icon, option.name))
            .alignment(ratatui::layout::HorizontalAlignment::Left)
            .style(style);
        let atalho = Paragraph::new(format!("{}", option.atalho))
            .alignment(ratatui::layout::HorizontalAlignment::Right)
            .style(Style::default().fg(Color::Red));
        frame.render_widget(widget, chunks[i]);
        frame.render_widget(atalho, chunks[i]);
    }
}

fn redner(frame: &mut ratatui::Frame) {
    let area = frame.area();
    let vertical = Layout::default()

        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20)

        ]).split(area);
    let hori = Layout::default()
        .direction(Direction::Horizontal)

        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),

        ]).split(vertical[1]);

    let block_ui = BlockUI {
        vertical: vertical,
        horizontal: hori,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));


    let inner = block.inner(block_ui.horizontal[1]);

    render_options(frame, inner);
}
