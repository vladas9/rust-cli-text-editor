use super::Editor;

pub struct History {
    states: Vec<Editor>,
    pub del_count: u16,
}

impl History {
    pub fn new() -> Self {
        Self {
            states: vec![Editor::new()],
            del_count: 0,
        }
    }

    pub fn save_state(&mut self, editor: &Editor) {
        self.states.push(editor.clone());
    }

    pub fn del_state(&mut self) {
        self.states.pop();
    }

    pub fn undo(&mut self, editor: &mut Editor) {
        if let Some(prev_state) = self.states.pop() {
            editor.buffer = prev_state.buffer;
            editor.cursor_x = prev_state.cursor_x;
            editor.cursor_y = prev_state.cursor_y;
        } else {
            self.states.push(Editor::new());
        }
    }
}
