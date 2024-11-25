use macroquad::color::{GRAY, RED, WHITE};
use macroquad::prelude::{draw_rectangle, draw_text_ex, Font, TextParams};
use macroquad::text::measure_text;
use crate::compiler::lexer::Token;
use crate::constants::{COMMENT_COLOR, IDENT_COLOR, KEYWORD_COLOR, NUMERIC_COLOR, OPERATOR_COLOR, STRING_COLOR, TOOLBAR_SIZE};
use crate::editor::texteditor::Textedit;

// draw contents from a textedit buffer
pub trait DrawTextedit {
    fn vert_gap(&self) -> f32;

    fn indent(&self) -> f32;

    fn font_size(&self) -> f32;

    fn mouse_wheel_y(&self) -> f32;

    fn textedit(&self) -> &Textedit;

    fn draw_tokens(&mut self, tokens: &Vec<Token>, text_params: &TextParams) {
        let mut x_offset = self.indent() + TOOLBAR_SIZE;
        let mut y_offset = self.vert_gap() - self.mouse_wheel_y();
        let mut params = text_params.clone();
        let mut contents: String;
        for token in tokens {
            match token {
                Token::Numeric(_) => {params.color = NUMERIC_COLOR;}
                Token::String(_) => {params.color = STRING_COLOR;}
                Token::Operator(_) => {params.color = OPERATOR_COLOR;}
                Token::Keyword(_) => {params.color = KEYWORD_COLOR;}
                Token::Ident(_) => {params.color = IDENT_COLOR;}
                Token::Comment(_) => {params.color = COMMENT_COLOR;}
                _ => {params.color = WHITE;}
            }
            contents = token.value();
            if contents == "\r" {continue;}
            if contents == "\n" {
                y_offset += self.font_size();
                x_offset = self.indent() + TOOLBAR_SIZE;
                continue;
            }
            draw_text_ex(&contents,
                         x_offset, y_offset,
                         params.clone());
            x_offset += measure_text(&contents,
                                     params.font,
                                     self.font_size() as u16,
                                     1.0).width;
        }
    }

    fn draw_contents(&mut self, contents: &str, params: &TextParams, spacing: Option<f32>) {
        // when keyword coloring is added, this will have to change
        let mut y_offset = self.vert_gap() - self.mouse_wheel_y();
        let mut x_offset = self.indent() + TOOLBAR_SIZE;
        let extra_space = if spacing.is_some() {
            spacing.unwrap()
        } else {
            1.0
        };
        for char in contents.chars() {
            if char == '\0' { break; }
            if char == '\r' { continue; }
            if char == '\n' {
                y_offset += self.font_size() * extra_space;
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

    fn draw_line_numbers(&mut self, contents: &str, params: &TextParams, spacing: Option<f32>) {
        let mut y: f32 = self.vert_gap() - self.mouse_wheel_y();
        let x = self.indent() / 2.0 - self.font_size() + TOOLBAR_SIZE;
        let extra_space = if spacing.is_some() {
            spacing.unwrap()
        } else {
            1.0
        };
        let mut params = params.clone();
        params.color = GRAY;
        let lines = contents.lines().count();
        for i in 1..=lines {
            draw_text_ex(&i.to_string(),
                         x, y,
                         params.clone());
            y += self.vert_gap() * extra_space;
        }
    }

    fn draw_cursor(&mut self, contents: &str, font: &Font, spacing: Option<f32>) {
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

        let extra_space = if spacing.is_some() {
            spacing.unwrap()
        } else {
            1.0
        };
        let range: &str = &contents[line_start..self.textedit().pointer];
        let chars_before: f32 = measure_text(range,
                                             font_option,
                                             self.font_size() as u16, 1.0).width;
        let offset = ptr_size.offset_y + self.mouse_wheel_y();

        let ptr_x: f32 = self.indent() + TOOLBAR_SIZE + chars_before;
        let ptr_y: f32 = (self.font_size() * (lines_before + 1) as f32 - offset) * extra_space;

        // todo: the cursor is slightly too low when double spacing is enabled
        draw_rectangle(ptr_x, ptr_y, // draws starting from what should be bottom.
                       2.0,
                       ptr_size.height,
                       RED);
    }
}