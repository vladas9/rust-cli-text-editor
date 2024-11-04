use super::{
    display::display_message_at_bottom,
    input::{save_file_input, wait_input},
    Editor,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    process,
};
use termion::terminal_size;

pub fn save_to_file(editor: &mut Editor) {
    if editor.filename.len() == 0 {
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
        editor.filename = filename;
    }

    match File::create(&editor.filename) {
        Ok(mut file) => {
            let buffer = editor.get_buffer();
            for line in buffer {
                if let Err(_) = writeln!(file, "{}", line) {
                    display_message_at_bottom(format!(
                        "Failed to write to file {}",
                        editor.filename
                    ));
                    wait_input();
                    return;
                }
            }

            display_message_at_bottom(format!("File {} saved successfully", editor.filename));
            wait_input();
        }
        Err(_) => {
            display_message_at_bottom(format!("Failed to create {}", editor.filename));
            wait_input();
        }
    }
}

pub fn load_from_file(editor: &mut Editor, path: String) {
    let file = File::open(&path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            editor.buffer.clear();
            for line in reader.lines() {
                match line {
                    Ok(text) => editor.buffer.push(text),
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                        process::exit(1);
                    }
                }
            }
            editor.filename = path;
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
