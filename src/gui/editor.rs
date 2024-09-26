use macroquad::color::{GRAY, RED, WHITE};
use macroquad::input::{get_last_key_pressed, KeyCode};
use macroquad::text::{draw_multiline_text, draw_text};
use crate::editor::texteditor::Textedit;

// todo: revamp rendering for text, write chars individually, or by word idk
// todo: get proper pointer pos on screen

pub struct EditorGui {
    textedit: Textedit,
    font_size: f32,
    indent: f32,
    spacing: f32,
    vert_gap: f32,
}

impl EditorGui {
    pub fn new() -> Self {
        Self {
            textedit: Textedit::new(),
            font_size: 30.0,
            indent: 50.0,
            spacing: 1.0,
            vert_gap: 30.0,
        }
    }

    pub fn draw(&mut self) {
        self.textedit.read().unwrap();
        let contents: &str = &String::from_utf8(self.textedit.buffer.clone()).unwrap();
        self.read_inputs(contents);
        self.draw_contents(contents);
        self.draw_pointer(contents);
        self.draw_line_numbers(contents);
    }

    fn draw_contents(&mut self, contents: &str) {
        // when keyword coloring is added, this will have to change
        draw_multiline_text(contents,
                            self.indent, self.vert_gap,
                            self.font_size,
                            Some(self.spacing),
                            WHITE);
    }

    fn draw_line_numbers(&mut self, contents: &str) {
        let lines: usize = contents.chars()
            .filter(|&c| c == '\n')
            .count() + 1;
        let mut y: f32 = self.vert_gap;
        for i in 1..=lines {
            draw_text(i.to_string().as_str(),
                      10.0,y,
                      self.font_size,
                      GRAY);
            y += self.font_size;
        }
    }

    fn draw_pointer(&mut self, contents: &str) {
        // draw a vertical line between chars (ascii 124),
        // with the first char being at the position of the pointer
        let lines_before: usize = contents[0..self.textedit.pointer]
            .chars()
            .filter(|&c| c == '\n').count();
        let chars_after: usize = contents[0..self.textedit.pointer]
            .chars()
            .rev()
            .collect::<String>()
            .chars()
            .position(|c| c== '\n').unwrap_or(self.textedit.pointer);

        // todo: find a way to ge the char_width, and char_height for the font.

        let ptr_x: f32 = (self.indent + self.font_size * chars_after as f32) / 2.0;
        let ptr_y: f32 = self.vert_gap + self.font_size * lines_before as f32;

        draw_text("$",
                  ptr_x, ptr_y,
                  self.font_size,
                  RED);
        println!("{:#?}", self.textedit.pointer);
    }

    fn read_inputs(&mut self, contents: &str) {
        let key = get_last_key_pressed();
        match key {
            Some(k) => self.parse_inputs(k, contents),
            _ => {}
        }
    }

    fn parse_inputs(&mut self, key: KeyCode, contents: &str) {
        match key {
            KeyCode::Right => {
                if self.textedit.pointer < contents.len() {
                    self.textedit.pointer += 1;
                }
            },
            KeyCode::Left => {
                if self.textedit.pointer > 0 {
                    self.textedit.pointer -= 1;
                }
            },
            _ => {}
        }
    }
}
