mod buffer;
mod command;
mod scenes;
mod ui;
mod utils;

use std::fs::File;
use std::io::stdout;
use std::path::PathBuf;

use crate::editor::{home::HomeScene, sidebar::Sidebar, ui::KynaUi};
use buffer::FileBuffer;
pub use command::*;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::Widget};
pub use scenes::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Modes {
    Normal,
    Insert,
    Select,
}

pub struct KynaEditor {
    ui: KynaUi,
}

impl KynaEditor {
    fn buffer_from(path: PathBuf) -> std::io::Result<FileBuffer> {
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

        Ok(FileBuffer::new(path, lines))
    }

    pub fn new(path: Option<String>) -> std::io::Result<Self> {
        let mut ui = KynaUi::new();
        if let Some(path) = path {
            let path = PathBuf::from(path);
            ui.add_scene(Sidebar::new(Self::buffer_from(path)?), true);
        } else {
            ui.add_scene(HomeScene::new(), true);
        }
        Ok(Self { ui })
    }

    ///Handles the provided `command` and returns if it should exit or not
    pub fn handle_command(&mut self, command: KynaCommand) -> bool {
        match command {
            KynaCommand::Exit => true,
            _ => false,
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;
        terminal.show_cursor()?;

        loop {
            terminal.draw(|frame| self.render(frame.area(), frame.buffer_mut()))?;
            terminal.show_cursor()?;
            if let Event::Key(key) = event::read()? {
                if self.ui.has_active_scene()
                    && let Some(scene) = self.ui.get_active_scene()
                {
                    let command = scene.handle_key(key);
                    if self.handle_command(command) {
                        break;
                    };
                }
            }
        }
        Ok(())
    }
}

impl Widget for &KynaEditor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        self.ui.render(area, buf);
    }
}
