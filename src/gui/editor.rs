use macroquad::color::{GRAY, WHITE};
use macroquad::text::{draw_multiline_text, draw_text};
use crate::editor::texteditor::Textedit;

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
        // write all text to the screen, on \n or its equivalent
        // jump down to new line by size + spacing
        self.textedit.read()
            .unwrap_or_else(
                |_| panic!("Error reading file: {}", self.textedit.file)
            );
        let contents: &str = &String::from_utf8(self.textedit.buffer.clone()).unwrap();
        draw_multiline_text(contents,
                            self.indent, self.vert_gap,
                            self.font_size,
                            Some(self.spacing),
                            WHITE);
        self.draw_line_numbers(contents);
    }

    fn draw_line_numbers(&mut self, contents: &str) {
        let lines: usize = contents.chars().filter(|&c| c == '\n').count() + 1;
        let mut y: f32 = self.vert_gap;
        for i in 1..=lines {
            draw_text(i.to_string().as_str(),
                      10.0,y,
                      self.font_size,
                      GRAY);
            y += self.font_size;
        }
    }
}