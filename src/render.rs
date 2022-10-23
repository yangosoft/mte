

use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};

use std::io::{stdout, Write};


pub struct Render {
    x: u32,
    y: u32
}

impl Render {
    

    pub fn new() -> Self {
        Self{
            x: 0,
            y: 0
        }
    }

    pub fn clear_screen(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn insert_char(&mut self, ch: char)
    {
        let s = String::from(ch);
        print!("{}",s);
        stdout().flush().unwrap();
        self.x += 1;        
    }

    pub fn insert_newline(&mut self)
    {
        self.y +=1;
        print!("\n");
        stdout().flush().unwrap();
    }

}