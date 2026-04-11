use crate::editor::KynaCommand;
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub mod file;
pub mod home;
pub mod sidebar;

pub trait KynaScene {
    fn handle_key(&mut self, _key: KeyEvent) -> KynaCommand {
        KynaCommand::None
    }
    fn render(&self, area: Rect, buffer: &mut Buffer);
}

///An ID to reference a Scene inside kyna editor
#[derive(Debug, Clone, Copy)]
pub struct SceneId(pub usize);
