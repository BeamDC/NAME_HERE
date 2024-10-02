use macroquad::input::{is_key_down, KeyCode};
use crate::editor::texteditor::Textedit;
use crate::gui::editor::EditorGui;

pub fn parse_inputs(editor: &mut EditorGui, key: KeyCode, contents: &str) {
    // remove newlines. to keep consistency with vector version
    // todo: when cursor is pushed past the line length, wrap to 0 on the next line.
    match key {
        KeyCode::Right => {
            if editor.textedit.pointer < contents.len() {
                editor.textedit.pointer += 1;
            }

        },
        KeyCode::Left => {
            if editor.textedit.pointer > 0 {
                editor.textedit.pointer -= 1;
            }
        },
        KeyCode::Down => {},
        KeyCode::Up => {},
        KeyCode::Delete => {
            if editor.textedit.buffer.len() > 0 {
                editor.textedit.buffer.remove(editor.textedit.pointer);
            }
        },
        KeyCode::Backspace => {
            if editor.textedit.pointer > 0 {
                editor.textedit.buffer.remove(editor.textedit.pointer - 1);
                editor.textedit.pointer -= 1;
            }
        },
        KeyCode::Enter => {
            let ch: u8 = 10;
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            // todo: properly handle a newline, jump down.
            editor.textedit.pointer += 1;
        }
        KeyCode::A => {
            let mut ch: u8 =  97;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::B => {
            let mut ch: u8 =  98;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::C => {
            let mut ch: u8 =  99;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::D => {
            let mut ch: u8 =  100;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::E => {
            let mut ch: u8 =  101;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::F => {
            let mut ch: u8 =  102;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::G => {
            let mut ch: u8 =  103;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::H => {
            let mut ch: u8 =  104;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::I => {
            let mut ch: u8 =  105;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::J => {
            let mut ch: u8 =  106;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::K => {
            let mut ch: u8 =  107;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::L => {
            let mut ch: u8 =  108;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::M => {
            let mut ch: u8 =  109;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::N => {
            let mut ch: u8 =  110;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::O => {
            let mut ch: u8 =  111;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::P => {
            let mut ch: u8 =  112;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::Q => {
            let mut ch: u8 =  113;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::R => {
            let mut ch: u8 =  114;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::S => {
            let mut ch: u8 =  115;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::T => {
            let mut ch: u8 =  116;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::U => {
            let mut ch: u8 =  117;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::V => {
            let mut ch: u8 =  118;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::W => {
            let mut ch: u8 =  119;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::X => {
            let mut ch: u8 =  120;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::Y => {
            let mut ch: u8 =  121;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        KeyCode::Z => {
            let mut ch: u8 =  122;
            if is_key_down(KeyCode::LeftShift) { ch ^= 1<<5; } // this makes me look smart
            editor.textedit.buffer.insert(
                editor.textedit.pointer,
                ch);
            editor.textedit.pointer += 1;
        }
        _ => {}
    }

    // todo: re-sync cursor and pointer, issue below
    // (pointer likes to got out of bounds when deleting down to 0 chars)
}