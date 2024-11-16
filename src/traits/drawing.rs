use macroquad::color::{GRAY, RED};
use macroquad::prelude::{draw_rectangle, draw_text_ex, Font, TextParams};
use macroquad::text::measure_text;
use crate::constants::TOOLBAR_SIZE;
use crate::editor::texteditor::Textedit;

// draw contents from a textedit buffer
pub trait DrawTextedit {
    fn vert_gap(&self) -> f32;
    fn indent(&self) -> f32;
    fn font_size(&self) -> f32;
    fn mouse_wheel_y(&self) -> f32;
    fn textedit(&self) -> &Textedit;
    fn draw_contents(&mut self, contents: &str, params: &TextParams, ) { // todo: make this a trait too!
        // when keyword coloring is added, this will have to change
        let mut y_offset = self.vert_gap() - self.mouse_wheel_y();
        let mut x_offset = self.indent() + TOOLBAR_SIZE;
        for char in contents.chars() {
            if char == '\0' { break; }
            if char == '\r' { continue; }
            if char == '\n' {
                y_offset += self.font_size();
                x_offset = self.indent() + TOOLBAR_SIZE;
                continue;
            }
            let char: &str = &char.to_string();
            draw_text_ex(char,
                         x_offset, y_offset,
                         params.clone());
            x_offset += measure_text(char, params.font, self.font_size() as u16, 1.0).width;
        }
    }

    fn draw_line_numbers(&mut self, contents: &str, params: &TextParams) {
        let mut y: f32 = self.vert_gap() - self.mouse_wheel_y();
        let x = self.indent() / 2.0 - self.font_size() + TOOLBAR_SIZE;
        let mut params = params.clone();
        params.color = GRAY;
        let lines = contents.lines().count();
        for i in 1..=lines {
            draw_text_ex(&i.to_string(),
                         x, y,
                         params.clone());
            y += self.vert_gap();
        }
    }

    fn draw_cursor(&mut self, contents: &str, font: &Font) {
        // todo: save the cursors pre clamp location
        // todo: so that the cursor, can jump back to it if it hits a line >= the re clamp size
        let font_option = Option::from(font);
        let ptr_size = measure_text("!",font_option, self.font_size().clone() as u16, 1.0);
        // line breaks before the pointer
        let lines_before: usize = contents[..self.textedit().pointer].chars().filter(|&c| c =='\n').count();
        // length of chars between last line break and the pointer
        // find first \n before pointer
        // measure text from \n to pointer - 1
        let mut line_start = contents[..self.textedit().pointer].rfind('\n').unwrap_or(0);
        if line_start != 0{
            line_start += 1;
        }
        let range: &str = &contents[line_start..self.textedit().pointer];
        let chars_before: f32 = measure_text(range,
                                             font_option,
                                             self.font_size() as u16, 1.0).width; // measure_text(char, params.font, self.font_size as u16, 1.0).width
        let ptr_x: f32 = self.indent() + TOOLBAR_SIZE + chars_before;
        let ptr_y: f32 = self.font_size() * (lines_before + 1) as f32 - ptr_size.offset_y - self.mouse_wheel_y();
        draw_rectangle(ptr_x, ptr_y, // draws starting from what should be bottom.
                       2.0,
                       ptr_size.height,
                       RED);
    }
}