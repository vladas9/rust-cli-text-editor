pub mod display;
pub mod file;
pub mod input;

pub struct Editor {
    filename: String,
    buffer: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            filename: String::new(),
            buffer: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn get_buffer(&self) -> &Vec<String> {
        &self.buffer
    }

    pub fn insert_text(&mut self, text: &str) {
        let line = &mut self.buffer[self.cursor_y];
        line.insert_str(self.cursor_x, text);
        self.cursor_x += text.len();
    }

    pub fn new_line(&mut self) {
        if self.cursor_y >= self.buffer.len() {
            self.buffer.push(String::new());
        } else {
            let current_line = &self.buffer[self.cursor_y];
            let new_line_content = current_line[self.cursor_x..].to_string();

            self.buffer.insert(self.cursor_y + 1, new_line_content);

            self.buffer[self.cursor_y].drain(self.cursor_x..);
        }
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            let line = &mut self.buffer[self.cursor_y];
            line.remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            let previous_line_length = self.buffer[self.cursor_y - 1].len();

            let line = self.buffer.remove(self.cursor_y);
            self.buffer[self.cursor_y - 1].insert_str(previous_line_length, &line);

            self.cursor_y -= 1;
            self.cursor_x = previous_line_length;
        }
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.buffer[self.cursor_y].len().min(self.cursor_x);
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y + 1 < self.buffer.len() {
            self.cursor_y += 1;
            self.cursor_x = self.buffer[self.cursor_y].len().min(self.cursor_x);
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_x < self.buffer[self.cursor_y].len() {
            self.cursor_x += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }
}
