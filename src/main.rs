mod editor;

mod render;

use editor::Editor;

use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    println!("Hello, world!");
    terminal::enable_raw_mode()?;
    let mut e = Editor::new();
    e.clear_screen();
    loop {
        e.run()?;    
    }
    
    Ok(())
}
