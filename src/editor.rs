use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use crossterm::terminal;
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
            reader: Reader,
            render: Render::new(),
        }
    }

    pub fn clear_screen(&mut self) {
        self.render.clear_screen();
    }

    fn draw_status_bar(&mut self) -> crossterm::Result<bool> {
        self.render.draw_status_bar()?;
        Ok(true)
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        let k = self.reader.read_key()?;

        match k {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => self.render.insert_newline(),
            KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => self.render.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(ch) => ch,
                _ => unreachable!(),
            }),
            KeyEvent {
                code:
                    direction
                    @
                    (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End),
                modifiers: KeyModifiers::NONE,
            } => self.render.move_cursor(direction),
            KeyEvent {
                code: key @ (KeyCode::Backspace | KeyCode::Delete),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(key, KeyCode::Delete) {
                    self.render.move_cursor(KeyCode::Right)
                }
                self.render.delete_char()
            }
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
            } => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        //self.output.refresh_screen()?;
        self.draw_status_bar()?;
        self.process_keypress()
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        self.render.clear_screen().expect("error");
    }
}
