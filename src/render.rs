use crossterm::event::KeyCode;
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, terminal};

use std::f32::consts::E;
use std::io::{stdout, Write};

use crate::linebuffer::LineBuffer;
use crate::log::Log;

use std::fmt;

pub struct Render {
    x: u16,
    y: u16,
    win_size: (usize, usize),
    lines: LineBuffer,
}

impl Render {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 2))
            .unwrap();
        let mut ln = LineBuffer::new();
        ln.insert_row(0, "".to_string());

        Self {
            x: 0,
            y: 0,
            win_size,
            lines: ln,
        }
    }

    fn render_line(&mut self, line_num: u16, line: &String) {
        Log(&fmt::format(format_args!(
            "Rendering line {} content: {}",
            line_num, line
        )));

        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveTo(0, line_num),
            terminal::Clear(ClearType::CurrentLine),
            Print(line),
            cursor::Show
        );
        stdout().flush().unwrap();
    }

    pub fn clear_screen(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn insert_char(&mut self, ch: char) {
        self.lines.insert_char(self.x as usize, self.y as usize, ch);
        let s = self.lines.get_line(self.y as usize).unwrap();
        self.render_line(self.y, &s);
        self.x += 1;
    }

    pub fn insert_newline(&mut self) {
        Log("insert_new_line");
        let s = fmt::format(format_args!("Num lines {}", self.lines.get_num_lines()));

        Log(&s);
        let content = self.lines.get_line(self.y as usize).unwrap();
        //if (self.y as usize) == self.lines.get_num_lines() - 1 {
        Log("Create one");
        self.y += 1;

        if content.len() == 0 || (self.x as usize) == content.len() {
            Log("Is empty!");
            self.x = 0;
            self.lines.insert_row(self.y as usize, String::new());
        } else if (self.x as usize) < content.len() {
            Log("Not empty!: ");
            let slice_content = &content[self.x as usize..];
            let slice_old_content = &content[..self.x as usize];
            let new_old_content = String::from(slice_old_content);
            self.lines
                .replace_row((self.y - 1) as usize, new_old_content.clone());
            self.render_line(self.y - 1, &new_old_content);
            let new_content = String::from(slice_content);
            Log(&new_content);
            self.x = 0;
            self.lines.insert_row(self.y as usize, new_content.clone());
            self.render_line(self.y, &new_content);

            self.render_until_end(self.y);
            /*for line_num in self.y..self.lines.get_num_lines() as u16 {
                let content = self.lines.get_line(line_num as usize).unwrap();
                self.render_line(line_num, &content);
            }*/
        }
    }

    fn render_until_end(&mut self, from: u16) {
        for line_num in from..self.lines.get_num_lines() as u16 {
            let content = self.lines.get_line(line_num as usize).unwrap();
            self.render_line(line_num, &content);
        }

        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveTo(0, self.lines.get_num_lines() as u16),
            terminal::Clear(ClearType::FromCursorDown),
            cursor::MoveTo(self.x, self.y),
            cursor::Show
        );
    }

    pub fn delete_char(&mut self) {
        if self.x > 0 {
            self.x -= 1;

            let mut l = self.lines.get_line(self.y as usize).unwrap();
            l.remove(self.x as usize);
            self.lines.replace_row(self.y as usize, l.clone());
            self.render_line(self.y, &l);
        } else if (self.y as usize) < self.lines.get_num_lines() && self.y > 0 {
            self.lines.remove_row(self.y as usize);
            self.y -= 1;
            self.x = self.lines.get_line(self.y as usize).unwrap().len() as u16;
            self.render_until_end(self.y);
        }
    }

    pub fn get_current_cursor(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.y > 0 {
                    self.y -= 1;
                    let n = self.lines.get_line(self.y as usize).unwrap().len() as u16;
                    self.x = n as u16;
                }
            }
            KeyCode::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            KeyCode::Down => {
                if (self.y as usize) < self.lines.get_num_lines() - 1 {
                    self.y += 1;
                    let n = self.lines.get_line(self.y as usize).unwrap().len() as u16;
                    self.x = n as u16;
                }
            }
            KeyCode::Right => {
                let n = self.lines.get_line(self.y as usize).unwrap().len() as u16;
                if self.x < n {
                    self.x += 1;
                }
            }
            KeyCode::End => {
                let n = self.lines.get_line(self.y as usize).unwrap().len();
                self.x = n as u16;
            }
            KeyCode::Home => self.x = 0,
            _ => unimplemented!(),
        }
    }

    pub fn draw_status_bar(&mut self) -> crossterm::Result<bool> {
        let pos = self.get_current_cursor();
        let num_lines = self.lines.get_num_lines();
        let mut s = fmt::format(format_args!(
            "F1 Exit | F2 New | F3 Search | F4 Open | F5 Save | Line: {} Char: {} Lines: {}",
            pos.1, pos.0, num_lines
        ));

        let menu_size = s.len();
        for n in 0..self.win_size.0 - menu_size {
            s.push(' ');
        }

        queue!(
            stdout(),
            cursor::MoveTo(0, self.win_size.1 as u16),
            SetForegroundColor(Color::DarkBlue),
            SetBackgroundColor(Color::Yellow),
            SetAttribute(Attribute::Bold),
            cursor::Hide,
            Print(s),
            ResetColor,
            SetAttribute(Attribute::Reset)
        )?;
        //print!("F1 Exit");
        queue!(stdout(), cursor::MoveTo(self.x, self.y), cursor::Show)?;

        stdout().flush()?;
        Ok(true)
    }
}
