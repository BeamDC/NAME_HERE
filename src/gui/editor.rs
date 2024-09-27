use macroquad::color::{GRAY, RED, WHITE};
use macroquad::input::{get_last_key_pressed, KeyCode};
use macroquad::text::{draw_text, draw_text_ex, measure_text, Font, TextParams};
use crate::editor::texteditor::Textedit;
use std::vec::Vec;

// todo: revamp rendering for text, write chars individually, or by word idk
// todo: get proper pointer pos on screen

pub struct EditorGui {
    textedit: Textedit,
    font_size: f32,
    indent: f32,
    spacing: f32,
    vert_gap: f32,
    font: Font,
}

impl EditorGui {
    pub fn new(font: Font) -> Self {
        Self {
            textedit: Textedit::new(),
            font_size: 30.0,
            indent: 100.0,
            spacing: 1.0,
            vert_gap: 30.0,
            font,
        }
    }

    pub fn draw(&mut self) {
        self.textedit.read().unwrap();
        let binding  = String::from_utf8(self.textedit.buffer.clone())
            .unwrap()
            .replace("\r", "");
        let contents: Vec<&str> = binding
            .split('\n')// split into lines
            .collect::<Vec<&str>>();

        let font = self.font.clone();
        let params: TextParams = TextParams {
            font: Option::from(&font),
            font_size: self.font_size.clone() as u16,
            color: WHITE,
            ..Default::default()
        };
        self.read_inputs(&binding);
        self.draw_contents(&contents, &params);
        self.draw_pointer(&contents);
        self.draw_line_numbers(&contents, &params);
    }

    fn draw_contents(&mut self, contents: &Vec<&str>, params: &TextParams) {
        // when keyword coloring is added, this will have to change
        let mut gap = self.vert_gap;
        for line in contents {
            draw_text_ex(line,
                         self.indent, gap,
                         params.clone());
            gap += self.font_size // get better spacing formula
        }
    }

    fn draw_line_numbers(&mut self, contents: &Vec<&str>, params: &TextParams) {
        let mut y: f32 = self.vert_gap;
        let mut params = params.clone();
        params.color = GRAY;
        for i in 1..=contents.len() {
            draw_text_ex(i.to_string().as_str(),
                      10.0,y,
                      params.clone());
            y += self.font_size;
        }
    }

    fn draw_pointer(&mut self, contents: &Vec<&str>) {
        // draw a pointer (rect with size of char in selected font)
        // let size = measure_text()
        println!("{:?}", self.textedit.pointer);
    }

    fn read_inputs(&mut self, contents: &String) {
        let key = get_last_key_pressed();
        match key {
            Some(k) => self.parse_inputs(k, contents),
            _ => {}
        }
    }

    fn parse_inputs(&mut self, key: KeyCode, contents: &String) {
        // remove newlines. to keep consistency with vector version
        // let text = contents.replace("\n", "");

        match key {
            KeyCode::Right => {
                if self.textedit.pointer.0 < contents.len() {
                    self.textedit.pointer.0 += 1;
                }
            },
            KeyCode::Left => {
                if self.textedit.pointer.0 > 0 {
                    self.textedit.pointer.0 -= 1;
                }
            },
            _ => {}
        }
        // solve pointer-y
        self.textedit.pointer.1 =
            contents[0..self.textedit.pointer.0]
                .chars()
                .filter(|&c| c == '\n')
                .count();
    }
}