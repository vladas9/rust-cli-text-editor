use std::env;

use editor::{display::display_editor, file::load_from_file, input::handle_input, Editor};

mod editor;

fn main() {
    let mut editor = Editor::new();
    if let Some(path) = env::args().nth(1) {
        load_from_file(&mut editor, path);
    }
    loop {
        display_editor(&editor);
        if handle_input(&mut editor) {
            break;
        }
    }
}
