use super::Editor;

pub struct History {
    states: Vec<Editor>,
    location: usize,
    pub delete_state: bool,
}

impl History {
    pub fn new() -> Self {
        Self {
            states: vec![Editor::new()],
            location: 0,
            delete_state: true,
        }
    }

    pub fn save_state(&mut self, editor: &Editor) {
        self.states.truncate(self.location + 1);
        self.states.push(editor.clone());
        self.location += 1;
    }

    pub fn del_state(&mut self) {
        self.states.pop();
        self.location = self.location.saturating_sub(1);
    }

    pub fn undo(&mut self, editor: &mut Editor) {
        self.save_state(editor);
        if self.delete_state {
            self.location = self.location.saturating_sub(2);
        } else {
            self.location = self.location.saturating_sub(1);
        }
        let prev_state = &self.states[self.location];
        editor.buffer = prev_state.buffer.clone();
        editor.cursor_x = prev_state.cursor_x;
        editor.cursor_y = prev_state.cursor_y;
    }

    pub fn redo(&mut self, editor: &mut Editor) {
        if self.location < self.states.len() - 1 {
            self.location += 1;
            let next_state = &self.states[self.location];
            editor.buffer = next_state.buffer.clone();
            editor.cursor_x = next_state.cursor_x;
            editor.cursor_y = next_state.cursor_y;
        } else {
            println!("No further redo steps available.");
        }
    }
}
