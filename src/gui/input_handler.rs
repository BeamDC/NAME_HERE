use crate::editor::texteditor::Textedit;
use crate::gui::gui::Gui;
use crate::gui::toolbar::{Icons, Toolbar};
use macroquad::color::{Color, WHITE};
use macroquad::input::{is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton};
use macroquad::prelude::{draw_rectangle_lines, draw_text_ex, measure_text};
use macroquad::shapes::draw_rectangle;
use macroquad::text::{Font, TextParams};
use std::cmp::max;

macro_rules! insert_u8 {
    ($e1: expr, $e2: expr) => {
        $e2.buffer.insert($e2.pointer, $e1);
    };
}

pub trait GlobalInputHandle {
    type GuiType: Gui;
    fn key(&self) -> Option<KeyCode>;
    fn textedit(&self) -> Textedit;
    // fn terminal(&self) -> Option<Terminal>;
    fn gui(&self) -> Self::GuiType;
    fn set_context(&mut self, new_context: Textedit);
    fn set_gui(&mut self, new_gui: Self::GuiType);
    fn handle_inputs(&mut self) {
        match self.key() {
            Some(k) => {
                if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                    self.parse_shift_inputs(k)
                } else if is_key_down(KeyCode::LeftControl) { // not so easy on the eyes
                    self.parse_control_inputs(k)
                } else if is_key_down(KeyCode::LeftAlt) { // not so easy on the eyes
                    self.parse_alt_inputs(k)
                } else {
                    self.parse_general_inputs(k)
                }
                return
            }
            _ => {}
        }

        // check mouse inputs
        let mouse_pos = mouse_position();

        // there's probably a way to avoid these if statements
        if is_mouse_button_pressed(MouseButton::Left) {
            self.mouse_input_left(mouse_pos);
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.mouse_input_right(mouse_pos);
        }
    }
    fn parse_control_inputs(&mut self, key: KeyCode) {
        let mut editor = self.textedit();
        match key {
            KeyCode::S => { // save the editor contents to the open file
                editor.write().unwrap();
            }
            KeyCode::D => { // duplicate the current line : WIP
                let ptr = editor.pointer;
                let buffer = &editor.buffer.clone();
                let line_start = buffer[..ptr]
                    .iter()
                    .rposition(|&c| c == b'\n')
                    .unwrap_or(0);
                let line_end = buffer[ptr..]
                    .iter()
                    .position(|&c| c == 10)
                    .unwrap_or(buffer.len() - 1);

                println!("start:{}\nend:{}\nbuf:{}", line_start, line_end, buffer.len());
                let line = &buffer[line_start..line_end];
                println!("{:?}", line);
                editor.buffer.insert(line_end, 10);
                editor.buffer.splice(line_end + 1..line_end + 1, line.iter().cloned());
                editor.pointer += line.len();
            }
            _ => {}
        }
        // when looking at EOF, take a step back
        if editor.pointer > editor.buffer.len() - 1 {
            editor.pointer -= 1;
        }

        // update editor
        self.set_context(editor);
    }
    fn parse_alt_inputs(&mut self, key: KeyCode) {
        let mut editor = self.textedit();
        match key {
            _ => {}
        }
        // when looking at EOF, take a step back
        if editor.pointer > editor.buffer.len() - 1 {
            editor.pointer -= 1;
        }

        // update editor
        self.set_context(editor);
    }
    fn parse_shift_inputs(&mut self, key: KeyCode) {
        let mut editor = self.textedit();
        match key {
            // TODO: this is for selection
            // KeyCode::Right => {
            // },
            // KeyCode::Left => {
            // },
            // KeyCode::Down => {
            // },
            // KeyCode::Up => {
            // },

            // numbers :(
            KeyCode::Key0 => {
                let ch: u8 = 41;
                // if next is ), then dont add, just move ptr
                let next = editor.pointer + 1;
                if next < editor.buffer.len() && !editor.buffer[next] == 41 {
                    insert_u8!(ch, editor);
                }
                editor.pointer += 1;
            }
            KeyCode::Key1 => {
                let ch: u8 = 33;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key2 => {
                let ch: u8 = 64;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key3 => {
                let ch: u8 = 35;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key4 => {
                let ch: u8 = 36;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key5 => {
                let ch: u8 = 37;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key6 => {
                let ch: u8 = 94;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key7 => {
                let ch: u8 = 38;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key8 => {
                let ch: u8 = 42;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key9 => {
                let ch: u8 = 40;
                // when starting parenthesis, if there is no closing to match, add a closer
                // if [ptr..end] does not have a closer before the next open paren
                let closed = editor.buffer[editor.pointer..editor.buffer.len() - 1]
                    .iter()
                    .filter(|&c| *c == 41)
                    .count();
                let open = editor.buffer[0..editor.pointer]
                    .iter()
                    .filter(|&c| *c == 40)
                    .count();
                let needs_close: bool = open >= closed;

                if needs_close {
                    insert_u8!(41, editor);
                }
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }

            // letters :(
            KeyCode::A | KeyCode::B | KeyCode::C |
            KeyCode::D | KeyCode::E | KeyCode::F |
            KeyCode::G | KeyCode::H | KeyCode::I |
            KeyCode::J | KeyCode::K | KeyCode::L |
            KeyCode::M | KeyCode::N | KeyCode::O |
            KeyCode::P | KeyCode::Q | KeyCode::R |
            KeyCode::S | KeyCode::T | KeyCode::U |
            KeyCode::V | KeyCode::W | KeyCode::X |
            KeyCode::Y | KeyCode::Z
            => {
                let ch: u8 = key as u16 as u8;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }

            // other important chars
            KeyCode::Minus => {
                let ch: u8 = 95;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Equal => {
                let ch: u8 = 43;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::LeftBracket => {
                let ch: u8 = 123;
                let closed = editor.buffer[editor.pointer..editor.buffer.len() - 1]
                    .iter()
                    .filter(|&c| *c == 125)
                    .count();
                let open = editor.buffer[0..editor.pointer]
                    .iter()
                    .filter(|&c| *c == 123)
                    .count();
                let needs_close: bool = open >= closed;
                if needs_close {
                    insert_u8!(125, editor);
                }
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::RightBracket => {
                let ch: u8 = 125;
                let next = editor.pointer + 1;
                if next < editor.buffer.len() && !editor.buffer[next] == 125 {
                    insert_u8!(ch, editor);
                }
                editor.pointer += 1;
            }
            KeyCode::Backslash => {
                let ch: u8 = 124;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Semicolon => {
                let ch: u8 = 58;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Apostrophe => {
                let ch: u8 = 34;
                let closed = editor.buffer[editor.pointer..editor.buffer.len() - 1]
                    .iter()
                    .filter(|&c| *c == 34)
                    .count();
                let open = editor.buffer[0..editor.pointer]
                    .iter()
                    .filter(|&c| *c == 34)
                    .count();
                let needs_close: bool = open >= closed;
                if needs_close {
                    insert_u8!(34, editor);
                }
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Comma => {
                let ch: u8 = 60;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Period => {
                let ch: u8 = 62;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Slash => {
                let ch: u8 = 63;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            _ => {}
        }

        // when looking at EOF, take a step back
        if editor.pointer >= editor.buffer.len() &&
            editor.buffer.len() != 0 {
            editor.pointer -= 1;
        }

        // update editor
        self.set_context(editor);
    }
    fn parse_general_inputs(&mut self, key: KeyCode) {
        let mut editor = self.textedit();
        match key {
            KeyCode::Right => {
                if editor.pointer < editor.buffer.len() - 1 {
                    editor.pointer += 1;
                }
            }
            KeyCode::Left => {
                // if self.gui().name() == "Terminal" {return}
                if editor.pointer > 0 {
                    editor.pointer -= 1;
                }
            }
            // todo: when moving down to last line, cursor always jumps to the end.
            KeyCode::Down => {
                let mut ptr = editor.pointer;
                let buffer = &editor.buffer;
                // pos of start of current line (last \n passed)
                let mut start: usize = buffer[..ptr].len() -
                    buffer[..ptr]
                        .iter()
                        .rposition(|&x| x == 10)
                        .unwrap_or(0);
                if start == buffer[0..ptr].len() { start = 0; }
                // distance from current line start to ptr
                let from_last = buffer[start..ptr].len();
                // dist to next newline
                let mut end = buffer[ptr..buffer.len()]
                    .iter()
                    .position(|&x| x == 10)
                    .unwrap_or(0)
                    + from_last
                    + 1; // off by one :)
                if ptr + end >= buffer.len() {
                    // println!("OUT OF BOUNDS!");
                    end = buffer.len() - 1 - ptr;
                }
                if buffer[ptr..ptr + end].iter().filter(|&&c| c == 10).count() > 1 {
                    // println!("MULTILINE LINE CROSS!");
                    end = buffer[ptr..ptr + end].iter().position(|&x| x == 10).unwrap();
                }
                ptr += end;
                // if ptr move crossed 2 newlines, move ptr back two
                editor.pointer = ptr;
            }
            KeyCode::Up =>
                {
                // if self.gui().name() == "Terminal" {return}
                let buffer = &editor.buffer;
                let current_line_start = buffer[..editor.pointer]
                    .iter()
                    .rposition(|&x| x == b'\n')
                    .map(|pos| pos + 1)
                    .unwrap_or(1);

                let prev_line_start = buffer[..current_line_start - 1]
                    .iter()
                    .rposition(|&x| x == b'\n')
                    .map(|pos| pos + 1)
                    .unwrap_or(0);

                let current_column = max(editor.pointer, 1) - current_line_start;
                let prev_line_length = current_line_start - prev_line_start - 1;
                let new_position = if current_column <= prev_line_length {
                    prev_line_start + current_column
                } else {
                    current_line_start - 1
                };

                editor.pointer = new_position;
            }

            KeyCode::Delete => {
                if editor.buffer.len() > 0 &&
                    editor.buffer[editor.pointer] != 0 {
                    editor.buffer.remove(editor.pointer);
                }
            }
            KeyCode::Backspace => {
                if self.gui().name() == "Terminal" {return}
                if editor.pointer > 0 {
                    editor.buffer.remove(editor.pointer - 1);
                    editor.pointer -= 1;
                }
            }
            KeyCode::Enter => {
                let ch: u8 = 10;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Space => {
                let ch: u8 = 32;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Tab => { // todo: crashes when placed at EOF in terminal
                let ch: u8 = 32;
                insert_u8!(ch, editor);
                insert_u8!(ch, editor);
                insert_u8!(ch, editor);
                insert_u8!(ch, editor);
                editor.pointer += 4;
            }

            // numbers :(
            KeyCode::Key0 => {
                let ch: u8 = 48;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key1 => {
                let ch: u8 = 49;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key2 => {
                let ch: u8 = 50;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key3 => {
                let ch: u8 = 51;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key4 => {
                let ch: u8 = 52;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key5 => {
                let ch: u8 = 53;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key6 => {
                let ch: u8 = 54;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key7 => {
                let ch: u8 = 55;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key8 => {
                let ch: u8 = 56;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Key9 => {
                let ch: u8 = 57;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }

            // letters :(
            KeyCode::A | KeyCode::B | KeyCode::C |
            KeyCode::D | KeyCode::E | KeyCode::F |
            KeyCode::G | KeyCode::H | KeyCode::I |
            KeyCode::J | KeyCode::K | KeyCode::L |
            KeyCode::M | KeyCode::N | KeyCode::O |
            KeyCode::P | KeyCode::Q | KeyCode::R |
            KeyCode::S | KeyCode::T | KeyCode::U |
            KeyCode::V | KeyCode::W | KeyCode::X |
            KeyCode::Y | KeyCode::Z
            => {
                let ch: u8 = (key as u16 as u8) | 1 << 5;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }

            // other important chars
            KeyCode::Minus => {
                let ch: u8 = 45;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Equal => {
                let ch: u8 = 61;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::LeftBracket => {
                let ch: u8 = 91;
                let closed = editor.buffer[editor.pointer..editor.buffer.len() - 1]
                    .iter()
                    .filter(|&c| *c == ch)
                    .count();
                let open = editor.buffer[0..editor.pointer]
                    .iter()
                    .filter(|&c| *c == 93)
                    .count();
                let needs_close: bool = open >= closed;
                if needs_close {
                    insert_u8!(93, editor);
                }
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::RightBracket => {
                let ch: u8 = 93;
                let next = editor.pointer + 1;
                if next < editor.buffer.len() && !editor.buffer[next] == 93 {
                    insert_u8!(ch, editor);
                }
                editor.pointer += 1;
            }
            KeyCode::Backslash => {
                let ch: u8 = 92;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Semicolon => {
                let ch: u8 = 59;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Apostrophe => {
                let ch: u8 = 39;
                let closed = editor.buffer[editor.pointer..editor.buffer.len() - 1]
                    .iter()
                    .filter(|&c| *c == 39)
                    .count();
                let open = editor.buffer[0..editor.pointer]
                    .iter()
                    .filter(|&c| *c == 39)
                    .count();
                let needs_close: bool = open >= closed;
                if needs_close {
                    insert_u8!(39, editor);
                }
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Comma => {
                let ch: u8 = 44;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Period => {
                let ch: u8 = 46;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            KeyCode::Slash => {
                let ch: u8 = 47;
                insert_u8!(ch, editor);
                editor.pointer += 1;
            }
            _ => {}
        }

        // guard so the user can modify process commands
        if self.gui().name() == "Terminal" {
            let last_newline = editor.buffer[..]
                .iter()
                .rposition(|&c| c == b'\n')
                .unwrap_or(editor.pointer);
            if editor.pointer <= last_newline {
                editor.pointer = last_newline + 1;
            }
        }

        // when looking at EOF, take a step back
        if editor.pointer >= editor.buffer.len() &&
            editor.buffer.len() != 0 {
            editor.pointer -= 1;
        }

        // update editor
        self.set_context(editor);
    }
    fn mouse_input_left(&mut self, (x, y): (f32, f32)) {
        let mut gui = self.gui();
        // println!("LEFT MOUSE PRESSED @ ({x}, {y})");
        self.set_gui(gui);
    }
    fn mouse_input_right(&mut self, (x, y): (f32, f32)) {
        let mut editor = self.gui();
        // println!("RIGHT MOUSE PRESSED @ ({x}, {y})");
        self.set_gui(editor);
    }
}

pub trait ToolbarHandle {
    fn get_toolbar(&self) -> Toolbar;
    fn set_toolbar(&mut self, toolbar: Toolbar);
    fn get_font(&self) -> Font;
    fn get_font_size(&self) -> f32;
    fn detect_icon_click(&mut self) {
        let mut toolbar = self.get_toolbar();
        if !is_mouse_button_pressed(MouseButton::Left) { return }
        if toolbar.hovered.is_none() { return }
        toolbar.selected = toolbar.hovered;
        println!("{:?}", toolbar.selected);
        self.set_toolbar(toolbar);
    }
    fn show_tooltip(&mut self, textonly: bool) {
        let toolbar = self.get_toolbar();
        if toolbar.hovered.is_none() { return }
        let (mx,my) = mouse_position();
        let font = self.get_font();
        // get name of the icon
        let name = match toolbar.hovered {
            Some(Icons::FileOpen) => "Open File",
            Some(Icons::Editor) => "Editor",
            Some(Icons::Terminal) => "Terminal",
            Some(Icons::Search) => "Search",
            Some(Icons::Settings) => "Settings",
            Some(Icons::Icon) => "lil guy :>",
            _ => {""}
        };

        let params = TextParams {
            font: Option::from(&font),
            font_size: self.get_font_size().clone() as u16,
            color: WHITE,
            ..std::default::Default::default()
        };

        let text_size = measure_text(name, Option::from(&font),
                                     self.get_font_size() as u16, 1.0);

        let width = 30.0 + text_size.width;
        let height = 10.0 + text_size.height;
        let center_x = (width - text_size.width) / 2.0;
        let center_y = (height - text_size.height) / 2.0;
        let pos_x = mx * 1.5;
        let pos_y = my-(height/2.0);
        if !textonly { // todo: look into rounded rectangle!!!
            draw_rectangle(
                pos_x,
                pos_y,
                width,
                height,
                Color::new(0.29, 0.29, 0.29, 1.0),
            );
            draw_rectangle_lines(
                pos_x,
                pos_y,
                width,
                height,
                2.0,
                Color::new(0.38, 0.38, 0.38, 1.0),
            );
        }
        draw_text_ex(
            name,
            pos_x + center_x,
            pos_y + center_y + self.get_font_size(),
            params.clone()
        );
        self.set_toolbar(toolbar);
    }
}
