use ratatui::{buffer::Buffer, widgets::Widget};

use crate::editor::KynaScene;

pub struct HomeScene {
    selected: usize,
}
struct HomeOption {
    name: &'static str,
    icon: char,
    shortcut: char,
}

impl HomeScene {
    pub fn new() -> Self {
        Self { selected: 0 }
    }
}

impl HomeScene {
    const DEFAULT_OPTIONS: &[HomeOption] = &[
        HomeOption {
            name: "Open file",
            icon: '→',
            shortcut: 'z',
        },
        HomeOption {
            name: "Create file",
            icon: '→',
            shortcut: 'b',
        },
        HomeOption {
            name: "Help",
            icon: '→',
            shortcut: 's',
        },
    ];
    fn render_menu(&self, area: Rect, buf: &mut Buffer) {
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

        title.render(layout[0], buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2); Self::DEFAULT_OPTIONS.len()])
            .split(layout[1]);

        for (i, option) in Self::DEFAULT_OPTIONS.iter().enumerate() {
            let is_selected = self.selected == i;

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

            label.render(inner_chunks[0], buf);
            shortcut.render(inner_chunks[1], buf);
        }
    }
}

impl KynaScene for HomeScene {}
impl Widget for HomeScene {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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
        block.render(horizontal[1], buf);

        self.render_menu(inner, buf);
    }
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
