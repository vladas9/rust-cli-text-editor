use crate::editor::Editor;
use std::io::{stdout, Write};

pub fn display_editor(editor: &Editor) {
    print!("\x1b[2J\x1b[H");

    for (i, line) in editor.get_buffer().iter().enumerate() {
        print!("\x1b[{};0H", i + 1);
        print!("\x1b[K");

        if i == editor.get_cursor_position().1 {
            print!("\x1b[38;5;250m {} \x1b[0m {}", i + 1, line);
        } else {
            print!("\x1b[38;5;250m {} \x1b[0m {}", i + 1, line);
        }
    }

    print!(
        "\x1b[{};{}H",
        editor.get_cursor_position().1 + 1,
        editor.get_cursor_position().0 + 5
    );
    stdout().flush().unwrap();
}
