

use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};

use std::io::stdout;


pub struct Render {
}

impl Render {
    

    pub fn new() -> Self {
        Self{}
    }

    pub fn clear_screen(&mut self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}