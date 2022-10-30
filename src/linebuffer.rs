use crate::log::Log;
use std::fmt;
struct Line {
    line_content: String,
    render: String,
}

impl Line {
    pub fn new(line_content: String, render: String) -> Self {
        Self {
            line_content,
            render,
        }
    }

    pub fn insert_char(&mut self, at: usize, ch: char) {
        self.line_content.insert(at, ch);
    }

    pub fn delete_char(&mut self, at: usize) {
        self.line_content.remove(at);
    }

    pub fn get_content(&mut self) -> String {
        self.line_content.clone()
    }
}

pub struct LineBuffer {
    line_contents: Vec<Line>,
}

impl LineBuffer {
    pub fn new() -> Self {
        Self {
            line_contents: Vec::new(),
        }
    }

    pub fn insert_row(&mut self, at: usize, contents: String) {
        Log(&fmt::format(format_args!("Insert row {}", at)));
        let line = Line::new(contents, String::new());
        self.line_contents.insert(at, line);
    }

    pub fn replace_row(&mut self, at: usize, contents: String) {
        Log(&fmt::format(format_args!("Replace row {}", at)));
        let line = Line::new(contents, String::new());

        if self.line_contents.len() > at {
            self.line_contents.remove(at);
        }
        self.line_contents.insert(at, line);
    }

    pub fn remove_row(&mut self, at: usize) -> String {
        Log(&fmt::format(format_args!("Removing row {}", at)));
        return self.line_contents.remove(at).line_content;
    }

    pub fn insert_char(&mut self, pos_x: usize, pos_y: usize, c: char) {
        if pos_y >= self.line_contents.len() {
            self.insert_row(pos_y, String::from(c));
        } else {
            self.line_contents[pos_y].insert_char(pos_x, c);
        }
    }

    pub fn get_line(&mut self, line_num: usize) -> Result<String, &'static str> {
        if line_num >= self.line_contents.len() {
            return Err("Index of out bounds");
        }

        Ok(self.line_contents[line_num].get_content().clone())
    }

    pub fn get_num_lines(&self) -> usize {
        return self.line_contents.len();
    }
}
