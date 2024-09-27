use macroquad::color::{GRAY, RED, WHITE};
use macroquad::input::{get_last_key_pressed, KeyCode};
use macroquad::text::{draw_text, draw_text_ex, measure_text, Font, TextParams};
use crate::editor::texteditor::Textedit;
use std::vec::Vec;
use macroquad::math::clamp;
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
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
            indent: 100.0, // no sane person writes enough code for this to overlap
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
        self.draw_pointer(&contents, &font);
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

    fn draw_pointer(&mut self, contents: &Vec<&str>, font: &Font) {
        // todo: save the pointers pre clamp location
        // todo: so that the pointer, can jump back to it if it hits a line >= the re clamp size
        let ptr_size = measure_text("!",Option::from(font), self.font_size.clone() as u16, 1.0);
        self.textedit.pointer.0 = clamp(self.textedit.pointer.0, 0, contents[self.textedit.pointer.1].len());
        let ptr_x = self.indent + ptr_size.width * self.textedit.pointer.0 as f32;
        let ptr_y = self. vert_gap * (self.textedit.pointer.1 + 1) as f32 - ptr_size.offset_y;

        draw_rectangle_lines(ptr_x, ptr_y,
                             2.0,
                             ptr_size.height,
                             2.0, RED);
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
            KeyCode::Down => {
                if self.textedit.pointer.1 < contents.chars().filter(|&c| c == '\n').count() {
                    self.textedit.pointer.1 += 1;
                }
            },
            KeyCode::Up => {
                if self.textedit.pointer.1 > 0 {
                    self.textedit.pointer.1 -= 1;
                }
            },
            _ => {}
        }
        // solve pointer-y
        // self.textedit.pointer.1 =
        //     contents[0..self.textedit.pointer.0]
        //         .chars()
        //         .filter(|&c| c == '\n')
        //         .count();
    }
}