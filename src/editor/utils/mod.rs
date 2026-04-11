use ratatui::style::Color;

use crate::editor::Modes;

pub fn normalze_mode(mode: Modes) -> &'static str {
    match mode {
        Modes::Normal => "NORMAL",
        Modes::Insert => "INSERT",
        Modes::Select => "SELECT",
    }
}
pub fn get_icon(file: &str) -> (&'static str, Color) {
    if file.ends_with(".rs") {
        ("", Color::Red)
    } else if file.ends_with(".py") {
        ("", Color::Yellow)
    } else if file.ends_with(".js") {
        ("", Color::Green)
    } else {
        ("", Color::Gray)
    }
}
