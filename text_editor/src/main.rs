use editor::{display::display_editor, input::handle_input, Editor};

mod editor;

fn main() {
    let mut editor = Editor::new();
    loop {
        display_editor(&editor);
        if handle_input(&mut editor) {
            break;
        }
    }
}
