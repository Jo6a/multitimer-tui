pub struct InputField {
    pub content: String,
    pub cursor_position: usize,
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
