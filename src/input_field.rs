pub struct InputField {
    pub content: String,
    pub cursor_position: usize,
    pub content_history: Vec<String>,
    pub history_position: usize,
}

impl Default for InputField {
    fn default() -> Self {
        Self::new()
    }
}

impl InputField {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            content_history: vec![],
            history_position: 0,
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.chars().count() {
            self.cursor_position += 1;
        }
    }

    pub fn move_history_up(&mut self) {
        if self.history_position == 0 {
            return;
        }
        self.history_position -= 1;
        self.content = self.content_history[self.history_position].clone();
        self.cursor_position = self.content.len();
    }

    pub fn move_history_down(&mut self) {
        if self.history_position >= self.content_history.len() - 1 {
            self.content.clear();
            self.cursor_position = 0;
            self.history_position = self.content_history.len();
            return;
        }
        self.history_position += 1;
        self.content = self.content_history[self.history_position].clone();
        self.cursor_position = self.content.len();
    }

    pub fn insert_char(&mut self, c: char) {
        let pos = self
            .content
            .char_indices()
            .nth(self.cursor_position)
            .map_or(self.content.len(), |(i, _)| i);
        self.content.insert(pos, c);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let pos = self
                .content
                .char_indices()
                .nth(self.cursor_position - 1)
                .map_or(0, |(i, _)| i);
            self.content.remove(pos);
            self.move_cursor_left();
        }
    }
}
