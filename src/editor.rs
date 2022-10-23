
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use std::time::Duration;
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
}


impl Editor {
    pub fn new() -> Self {
        Self {
            reader : Reader
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {

        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        //self.output.refresh_screen()?;
        self.process_keypress()
    }
}