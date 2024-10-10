use std::cmp::max;
use macroquad::input::{is_key_down, KeyCode};
use crate::editor::texteditor::Textedit;

macro_rules! insert_u8 {
    ($e1: expr, $e2: expr) => {
        $e2.buffer.insert($e2.pointer, $e1);
    };
}

pub fn parse_control_inputs(editor: &mut Textedit, key: KeyCode) {
    match key {
        KeyCode::S => { // save the editor contents to the open file
            editor.write().unwrap();
        },
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
            editor.buffer.splice(line_end+1..line_end+1, line.iter().cloned());
            editor.pointer += line.len();
        },
        _ => {}
    }
    // when looking at EOF, take a step back
    if editor.pointer > editor.buffer.len() - 1{
        editor.pointer -= 1;
    }
}

pub fn parse_alt_inputs(editor: &mut Textedit, key: KeyCode) {
    match key {
        _ => {}
    }
    // when looking at EOF, take a step back
    if editor.pointer > editor.buffer.len() - 1{
        editor.pointer -= 1;
    }
}

pub fn parse_general_inputs(editor: &mut Textedit, key: KeyCode) {
    match key {
        KeyCode::Right => {
            if editor.pointer < editor.buffer.len() - 1 {
                editor.pointer += 1;
            }
        },
        KeyCode::Left => {
            if editor.pointer > 0 {
                editor.pointer -= 1;
            }
        },
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
            if ptr+end >= buffer.len() {
                // println!("OUT OF BOUNDS!");
                end = buffer.len() - 1 - ptr;
            }
            if buffer[ptr..ptr+end].iter().filter(|&&c| c == 10).count() > 1 {
                // println!("MULTILINE LINE CROSS!");
                end = buffer[ptr..ptr+end].iter().position(|&x| x == 10).unwrap();
            }
            ptr += end;
            // if ptr move crossed 2 newlines, move ptr back two
            editor.pointer = ptr;
        },
        KeyCode::Up => {
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
        },

        KeyCode::Delete => {
            if editor.buffer.len() > 0 &&
                editor.buffer[editor.pointer] != 0 {
                editor.buffer.remove(editor.pointer);
            }
        },
        KeyCode::Backspace => {
            if editor.pointer > 0 {
                editor.buffer.remove(editor.pointer - 1);
                editor.pointer -= 1;
            }
        },
        KeyCode::Enter => {
            let ch: u8 = 10;
            insert_u8!(ch, editor);
            editor.pointer += 1;
        }
        KeyCode::Space => {
            let ch: u8 = 32;
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Tab => { // todo: crashes when inserting tab at EOF
            let ch: u8 = 32;
            insert_u8!(ch, editor);
            insert_u8!(ch, editor);
            insert_u8!(ch, editor);
            insert_u8!(ch, editor);
            editor.pointer += 4;
            println!("ptr: {}\nlen: {}", editor.pointer, editor.buffer.len());
        },

        // numbers :(
        KeyCode::Key0 => {
            let mut ch: u8 =  48;
            if is_key_down(KeyCode::LeftShift) { ch = 41; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key1 => {
            let mut ch: u8 =  49;
            if is_key_down(KeyCode::LeftShift) { ch = 33; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key2 => {
            let mut ch: u8 =  50;
            if is_key_down(KeyCode::LeftShift) { ch = 64; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key3 => {
            let mut ch: u8 =  51;
            if is_key_down(KeyCode::LeftShift) { ch = 35; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key4 => {
            let mut ch: u8 =  52;
            if is_key_down(KeyCode::LeftShift) { ch = 36; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key5 => {
            let mut ch: u8 =  53;
            if is_key_down(KeyCode::LeftShift) { ch = 37; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key6 => {
            let mut ch: u8 = 54;
            if is_key_down(KeyCode::LeftShift) { ch = 94; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key7 => {
            let mut ch: u8 = 55;
            if is_key_down(KeyCode::LeftShift) { ch = 38; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key8 => {
            let mut ch: u8 = 56;
            if is_key_down(KeyCode::LeftShift) { ch = 42; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Key9 => {
            let mut ch: u8 = 57;
            if is_key_down(KeyCode::LeftShift) { ch = 40; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },

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
            let mut ch: u8 =  (key as u16 as u8) | 1<<5;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },

        // other important chars
        KeyCode::Minus => {
            let mut ch: u8 = 45;
            if is_key_down(KeyCode::LeftShift) { ch = 95; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Equal => {
            let mut ch: u8 = 61;
            if is_key_down(KeyCode::LeftShift) { ch = 43; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::LeftBracket => {
            let mut ch: u8 = 91;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::RightBracket => {
            let mut ch: u8 = 93;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Backslash => {
            let mut ch: u8 =  92;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        }
        KeyCode::Semicolon => {
            let mut ch: u8 =  59;
            if is_key_down(KeyCode::LeftShift) { ch = 58; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Apostrophe => {
            let mut ch: u8 = 39;
            if is_key_down(KeyCode::LeftShift) { ch = 34; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Comma => {
            let mut ch: u8 = 44;
            if is_key_down(KeyCode::LeftShift) { ch = 60; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Period => {
            let mut ch: u8 = 46;
            if is_key_down(KeyCode::LeftShift) { ch = 62; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        KeyCode::Slash => {
            let mut ch: u8 = 47;
            if is_key_down(KeyCode::LeftShift) { ch = 63; }
            insert_u8!(ch, editor);
            editor.pointer += 1;
        },
        _ => {}
    }

    // when looking at EOF, take a step back
    if editor.pointer >= editor.buffer.len() &&
        editor.buffer.len() != 0{
        editor.pointer -= 1;
    }
}