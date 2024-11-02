use super::Editor;
use std::io::{stdout, Write};
use termion::terminal_size;

fn get_render_line_size_and_offset(editor: &Editor) -> (usize, usize) {
    let term_height = terminal_size().unwrap().1 as usize;
    let cursor_y = editor.get_cursor_position().1;
    let buffer_len = editor.get_buffer().len();

    if cursor_y < term_height {
        (term_height.min(buffer_len), 0)
    } else {
        let offset = cursor_y.saturating_sub(term_height - 1);
        (term_height, offset)
    }
}

pub fn display_editor(editor: &Editor) {
    print!("\x1b[2J\x1b[H");

    let (render_size, offset) = get_render_line_size_and_offset(&editor);
    let line_number_width = 5;

    for (screen_line, (i, line)) in editor
        .get_buffer()
        .iter()
        .enumerate()
        .skip(offset)
        .take(render_size)
        .enumerate()
    {
        print!("\x1b[{};0H\x1b[K", screen_line + 1);

        if i == editor.get_cursor_position().1 {
            print!(
                "\x1b[38;5;250m {:>width$} \x1b[0m {}",
                i + 1,
                line,
                width = line_number_width
            );
        } else {
            print!(
                "\x1b[38;5;250m {:>width$} \x1b[0m {}",
                i + 1,
                line,
                width = line_number_width
            );
        }
    }

    let screen_cursor_y = editor.get_cursor_position().1 - offset + 1;
    print!(
        "\x1b[{};{}H",
        screen_cursor_y,
        editor.get_cursor_position().0 + 4 + line_number_width
    );

    stdout().flush().unwrap();
}

pub fn display_message_at_bottom(msg: String) {
    let term_height = terminal_size().unwrap().1;
    print!("\x1b[{};1H", term_height);
    print!("\x1b[K");
    print!("{}", msg);
    stdout().flush().unwrap();
}
