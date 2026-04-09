use std::path::PathBuf;

use git2::Repository;

use crate::editor::Modes;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub struct Buffer {
    pub name: PathBuf,
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub mode: Modes,
}

impl Buffer {
    pub fn new(name: PathBuf, lines: Vec<String>) -> Self {
        Self {
            name,
            lines,
            cursor: Cursor { x: 0, y: 0 },
            mode: Modes::Normal,
        }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn current_line(&self) -> &str {
        &self.lines[self.cursor.y as usize]
    }

    pub fn insert_char(&mut self, c: char) {
        let y = self.cursor.y as usize;
        let x = self.cursor.x as usize;
        let byte_idx = char_to_byte_idx(&self.lines[y], x);
        self.lines[y].insert(byte_idx, c);
        self.cursor.x += 1;
    }

    pub fn handle_enter(&mut self) {
        let x = self.cursor.x as usize;
        let y = self.cursor.y as usize;

        let before = self.lines[y][..x].to_string();
        let after = self.lines[y][x..].to_string();

        self.lines[y] = before;
        self.lines.insert(y + 1, after);

        self.cursor.y += 1;
        self.cursor.x = 0;
    }

    pub fn handle_backspace(&mut self) {
        let x = self.cursor.x as usize;
        let y = self.cursor.y as usize;

        if x > 0 {
            self.cursor.x -= 1;
            let byte_idx = char_to_byte_idx(&self.lines[y], self.cursor.x as usize);
            self.lines[y].remove(byte_idx);
        } else if y > 0 {
            let current = self.lines.remove(y);
            self.cursor.y -= 1;
            let prev_y = self.cursor.y as usize;
            self.cursor.x = self.lines[prev_y].chars().count() as u16;
            self.lines[prev_y].push_str(&current);
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.clamp_x();
        }
    }

    pub fn move_down(&mut self) {
        if (self.cursor.y as usize) < self.len() - 1 {
            self.cursor.y += 1;
            self.clamp_x();
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.lines[self.cursor.y as usize].chars().count() as u16;
        }
    }

    pub fn move_right(&mut self) {
        let line_len = self.current_line().chars().count() as u16;
        if self.cursor.x < line_len {
            self.cursor.x += 1;
        } else if (self.cursor.y as usize) < self.len() - 1 {
            self.cursor.y += 1;
            self.cursor.x = 0;
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let content = self.lines.join("\n");
        std::fs::write(&self.name, content)
    }

    fn clamp_x(&mut self) {
        let line_len = self.lines[self.cursor.y as usize].chars().count() as u16;
        if self.cursor.x > line_len {
            self.cursor.x = line_len;
        }
    }
    pub fn get_breanch(&self) -> Option<String> {
        let repo = Repository::open(self.name.as_path()).ok()?;
        let head = repo.head().ok()?;

        head.shorthand().map(|s| s.to_string())
    }
}

fn char_to_byte_idx(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len())
}
