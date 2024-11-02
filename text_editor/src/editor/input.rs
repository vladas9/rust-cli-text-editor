use std::io::{stdin, stdout, Read};

use termion::raw::IntoRawMode;

use super::Editor;

pub fn handle_input(editor: &mut Editor) -> bool {
    let _stdout = stdout().into_raw_mode().unwrap();
    let mut buf = [0; 1];
    stdin().read_exact(&mut buf).unwrap();
    match buf[0] {
        b' ' | b'a'..=b'z' => {
            let ch = buf[0] as char;
            editor.insert_text(&ch.to_string());
        }
        b'\x03' => {
            return true;
        }
        b'\r' => {
            editor.new_line();
        }
        b'\x7F' => {
            println!("Back");
            editor.delete_char();
        }
        b'\x1b' => {
            let mut seq = [0; 2];
            stdin().read_exact(&mut seq).unwrap();
            match seq {
                [b'[', b'A'] => editor.move_cursor_up(),
                [b'[', b'B'] => editor.move_cursor_down(),
                [b'[', b'C'] => editor.move_cursor_right(),
                [b'[', b'D'] => editor.move_cursor_left(),
                _ => {}
            }
        }
        _ => {}
    }
    false
}
