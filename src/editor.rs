
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use std::time::Duration;

use crate::render::Render;

struct Reader;

impl Reader {
    fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(300))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
pub struct Editor {
    reader: Reader,
    render: Render,
}


impl Editor {
    pub fn new() -> Self {
        Self {
            reader : Reader,
            render: Render::new()
        }
        
    }

    pub fn clear_screen(&mut self)
    {
        self.render.clear_screen();
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        let k = self.reader.read_key()?;
        print!("{:?}",k.code);

        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        //self.output.refresh_screen()?;
        self.process_keypress()
    }
}