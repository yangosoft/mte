use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, terminal};

use std::io::{stdout, Write};

use crate::linebuffer::LineBuffer;

pub struct Render {
    x: u16,
    y: u16,
    win_size: (usize, usize),
    lines: LineBuffer
}

impl Render {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 2))
            .unwrap();
        Self {
            x: 0,
            y: 0,
            win_size,
            lines: LineBuffer::new()
        }
    }

    pub fn clear_screen(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn insert_char(&mut self, ch: char) {
        queue!(stdout(), cursor::MoveTo(self.x, self.y), cursor::Show);
        stdout().flush().unwrap();
        let s = String::from(ch);
        print!("{}", s);
        stdout().flush().unwrap();
        self.lines.insert_char(self.x as usize, self.y as usize, ch);
        self.x += 1;
    }

    pub fn insert_newline(&mut self) {
        self.y += 1;
        self.x = 0;
        print!("\n");
        stdout().flush().unwrap();
        self.lines.insert_row(self.y as usize, String::new())
    }

    pub fn draw_status_bar(&mut self) -> crossterm::Result<bool> {
        /*let menu = "F1 Exit | F2 New | F3 Search | F4 Open | F5 Save ";
        let fill = self.win_size.1 - menu.len();

        let mut s = std::iter::repeat(" ")
            .take(fill)
            .collect::<String>();
        s = menu.to_owned() + &s;
        */

        let mut s: String = "F1 Exit | F2 New | F3 Search | F4 Open | F5 Save ".to_string();
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
