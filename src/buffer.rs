use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn read_buf(file_arg: &str) {
    let content: String = fs::read_to_string(file_arg).expect("erro");
}
