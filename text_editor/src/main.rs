use std::env;

use editor::{
    display::display_editor, file::load_from_file, input::handle_input, undo_redo::History, Editor,
};

mod editor;

fn main() {
    let mut editor = Editor::new();
    let mut history = History::new();
    if let Some(path) = env::args().nth(1) {
        load_from_file(&mut editor, path);
    }
    loop {
        display_editor(&editor);
        if handle_input(&mut editor, &mut history) {
            break;
        }
    }
}
