use super::{
    display::display_message_at_bottom,
    input::{save_file_input, wait_input},
    Editor,
};
use std::{fs::File, io::Write};
use termion::terminal_size;

pub fn save_to_file(editor: &Editor) {
    let term_height = terminal_size().unwrap().1;

    let mut filename = String::new();
    loop {
        display_message_at_bottom(format!("Enter filename to save: {}", filename));
        if save_file_input(&mut filename) {
            break;
        }
    }
    if filename.len() == 0 {
        return;
    }
    print!("\x1b[{};1H", term_height);
    print!("\x1b[K");

    match File::create(&filename) {
        Ok(mut file) => {
            let buffer = editor.get_buffer();
            for line in buffer {
                if let Err(_) = writeln!(file, "{}", line) {
                    display_message_at_bottom(format!("Failed to write to file {}", filename));
                    wait_input();
                    return;
                }
            }

            display_message_at_bottom(format!("File {} saved successfully", filename));
            wait_input();
        }
        Err(_) => {
            display_message_at_bottom(format!("Failed to create {}", filename));
            wait_input();
        }
    }
}
