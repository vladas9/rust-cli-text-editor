use super::{file::save_to_file, undo_redo::History, Editor};
use std::{
    char,
    io::{stdin, stdout, Read},
};
use termion::raw::IntoRawMode;

pub fn handle_input(editor: &mut Editor, history: &mut History) -> bool {
    let _stdout = stdout().into_raw_mode().unwrap();
    let buf = char_input::<1>();
    println!("key: {:?}", buf);
    match buf[0] {
        b' '..=b'~' => {
            if buf[0] == b' ' {
                history.save_state(editor);
            }
            let ch = buf[0] as char;
            editor.insert_text(&ch.to_string());
        }
        b'\x03' => {
            return true;
        }
        b'\r' => {
            history.save_state(editor);
            editor.new_line();
        }
        b'\x7F' => {
            if history.del_count == 0 {
                history.save_state(editor);
            }
            history.save_state(editor);
            history.del_count += 1;

            let chr = editor.delete_char();
            if chr != ' ' && chr != '\n' {
                history.del_state();
            }
        }
        b'\x13' => {
            save_to_file(editor);
        }
        b'\x1a' => {
            history.undo(editor);
            history.del_count = 0;
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

pub fn save_file_input(filename: &mut String) -> bool {
    let buf = char_input::<1>();
    match buf[0] {
        b' '..=b'~' => {
            let chr = char::from(buf[0]);
            filename.push(chr);
        }
        b'\r' => {
            return true;
        }
        b'\x7F' => {
            filename.pop();
        }
        b'\x1b' => {
            filename.clear();
            return true;
        }
        _ => {}
    }
    false
}

fn char_input<const N: usize>() -> [u8; N] {
    let mut buf = [0; N];
    stdin().read_exact(&mut buf).unwrap();
    buf
}

pub fn wait_input() {
    char_input::<1>();
}
