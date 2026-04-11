use crate::editor::KynaCommand;
use crossterm::event::KeyEvent;

pub mod file;
pub mod home;
pub mod sidebar;
pub trait KynaScene: ratatui::widgets::Widget {
    fn handle_key(&mut self, _key: KeyEvent) -> KynaCommand {
        KynaCommand::None
    }
}

///An ID to reference a Scene inside kyna editor
#[derive(Debug, Clone, Copy)]
pub struct SceneId(pub usize);
