use std::vec;

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

    pub fn get_content(&mut self) -> String
    {
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
        let mut line = Line::new(contents, String::new());
        self.line_contents.insert(at, line);
    }

    pub fn insert_char(&mut self, pos_x: usize, pos_y: usize, c: char) {
        if pos_y >= self.line_contents.len() {
            self.insert_row(pos_y, String::from(c));
        } else {
            self.line_contents[pos_y].insert_char(pos_x, c);
        }
    }
}
