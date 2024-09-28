use macroquad::color::{GRAY, RED, WHITE};
use macroquad::input::{get_last_key_pressed, mouse_wheel, KeyCode};
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};
use crate::editor::texteditor::Textedit;
use std::vec::Vec;
use macroquad::math::clamp;
use macroquad::shapes::{draw_rectangle};
// todo: revamp rendering for text, write chars individually, or by word idk
// todo: get proper cursor pos on screen

pub struct EditorGui {
    textedit: Textedit,
    font_size: f32,
    indent: f32,
    spacing: f32,
    vert_gap: f32,
    mouse_x: f32,
    mouse_y: f32,
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
            mouse_x: 0.0,
            mouse_y: 0.0,
            font,
        }
    }

    pub fn draw(&mut self) {
        // todo: jump to cursor command / hotkey
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
        self.mouse_y -= mouse_wheel().1;
        self.mouse_y = clamp(self.mouse_y, 0.0,1.0e6);
        self.read_inputs(&contents);
        self.draw_contents(&contents, &params);
        self.draw_cursor(&contents, &font);
        self.draw_line_numbers(&contents, &params);
    }

    fn draw_contents(&mut self, contents: &Vec<&str>, params: &TextParams) {
        // when keyword coloring is added, this will have to change
        // todo: scrolling
        let mut gap = self.vert_gap - self.mouse_y;
        for line in contents {
            draw_text_ex(line,
                         self.indent, gap,
                         params.clone());
            gap += self.font_size
        }
    }

    fn draw_line_numbers(&mut self, contents: &Vec<&str>, params: &TextParams) {
        let mut y: f32 = self.vert_gap - self.mouse_y;
        let mut params = params.clone();
        params.color = GRAY;
        for i in 1..=contents.len() {
            draw_text_ex(i.to_string().as_str(),
                      10.0,y,
                      params.clone());
            y += self.font_size;
        }
    }

    fn draw_cursor(&mut self, contents: &Vec<&str>, font: &Font) {
        // todo: save the cursors pre clamp location
        // todo: so that the cursor, can jump back to it if it hits a line >= the re clamp size
        let ptr_size = measure_text("!",Option::from(font), self.font_size.clone() as u16, 1.0);
        self.textedit.cursor.0 = clamp(self.textedit.cursor.0, 0, contents[self.textedit.cursor.1].len());
        let ptr_x = self.indent + ptr_size.width * self.textedit.cursor.0 as f32;
        let ptr_y = self.font_size * (self.textedit.cursor.1 + 1) as f32 - ptr_size.offset_y - self.mouse_y;

        draw_rectangle(ptr_x, ptr_y,
                       2.0,
                       ptr_size.height,
                       RED);
    }

    fn read_inputs(&mut self, contents:  &Vec<&str>) {
        let key = get_last_key_pressed();
        match key {
            Some(k) => self.parse_inputs(k, contents),
            _ => {}
        }
    }

    fn parse_inputs(&mut self, key: KeyCode, contents: &Vec<&str>) {
        // remove newlines. to keep consistency with vector version
        // todo: when cursor is pushed past the line length, wrap to 0 on the next line.
        let contents_str = contents.join("\n");
        match key {
            KeyCode::Right => {
                if self.textedit.cursor.0 < contents_str.len() {
                    self.textedit.cursor.0 += 1;
                }
            },
            KeyCode::Left => {
                if self.textedit.cursor.0 > 0 {
                    self.textedit.cursor.0 -= 1;
                }
            },
            KeyCode::Down => {
                if self.textedit.cursor.1 < contents_str.chars().filter(|&c| c == '\n').count() {
                    self.textedit.cursor.1 += 1;
                }
            },
            KeyCode::Up => {
                if self.textedit.cursor.1 > 0 {
                    self.textedit.cursor.1 -= 1;
                }
            },
            KeyCode::A | KeyCode::B => {

            }
            _ => {}
        }
        println!("{:#?}", self.textedit.buffer);
    }

    fn push_char(&mut self, key: KeyCode) {
        match key {
            KeyCode::A => {
                self.textedit.buffer.insert(self.textedit.pointer, 65);
            },
            KeyCode::B => {
                self.textedit.buffer.insert(self.textedit.pointer, 66);
            },
            _ => {}
        }
        println!("{:?}", self.textedit.buffer);
    }
}