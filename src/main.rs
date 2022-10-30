mod editor;

mod render;

mod linebuffer;

mod log;

use editor::Editor;

use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let mut e = Editor::new();
    e.clear_screen();
    while e.run()? {}
    
    Ok(())
}
