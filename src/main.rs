mod editor;
use editor::Editor;

fn main() {
    println!("Hello, world!");
    let mut e = Editor::new();
    e.run();
}
