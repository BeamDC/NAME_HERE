use crate::constants::TOOLBAR_SIZE;
use crate::editor::texteditor::Textedit;
use crate::traits::gui::Gui;
use crate::traits::input_handler::GlobalInputHandle;
use macroquad::color::{GRAY, RED, WHITE};
use macroquad::input::{get_last_key_pressed, mouse_wheel, KeyCode};
use macroquad::math::clamp;
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};

#[derive(Clone)]
pub struct EditorGui {
    pub textedit: Textedit,
    font_size: f32,
    font: Font,
    indent: f32,
    vert_gap: f32,
    _mouse_wheel_x: f32,
    mouse_wheel_y: f32,
    key: Option<KeyCode>,
}

impl EditorGui {
    pub fn new(font: Font) -> Self {
        let font_size = 30.0;
        Self {
            textedit: Textedit::new(),
            font_size,
            font,
            indent: font_size * 3.0,
            vert_gap: 30.0,
            _mouse_wheel_x: 0.0, // this can help with trackpad support!
            mouse_wheel_y: 0.0,
            key: None,
        }
    }

    pub fn draw(&mut self) {
        // todo: jump to cursor command / hotkey
        self.read_inputs();

        let contents  = String::from_utf8(self.textedit.buffer.clone())
            .unwrap();

        // TODO: THIS !!!!!!!
        // let tokens = tokenize(&contents);
        // TODO: THIS !!!!!!!

        let font = self.font.clone();
        let params: TextParams = TextParams {
            font: Option::from(&font),
            font_size: self.font_size.clone() as u16,
            color: WHITE,
            ..Default::default()
        };
        self.mouse_wheel_y -= mouse_wheel().1;
        self.mouse_wheel_y = clamp(self.mouse_wheel_y, 0.0, f32::MAX);

        self.draw_contents(&contents, &params);
        self.draw_cursor(&contents, &font);
        self.draw_line_numbers(&contents, &params);
    }
    fn draw_contents(&mut self, contents: &str, params: &TextParams, ) { // todo: make this a trait too!
        // when keyword coloring is added, this will have to change
        let mut y_offset = self.vert_gap - self.mouse_wheel_y;
        let mut x_offset = self.indent + TOOLBAR_SIZE;
        for char in contents.chars() {
            if char == '\0' { break; }
            if char == '\r' { continue; }
            if char == '\n' {
                y_offset += self.font_size;
                x_offset = self.indent + TOOLBAR_SIZE;
                continue;
            }
            let char: &str = &char.to_string();
            draw_text_ex(char,
                        x_offset, y_offset,
                        params.clone());
            x_offset += measure_text(char, params.font, self.font_size as u16, 1.0).width;
        }
    }

    fn draw_line_numbers(&mut self, contents: &str, params: &TextParams) {
        let mut y: f32 = self.vert_gap - self.mouse_wheel_y;
        let x = self.indent / 2.0 - self.font_size + TOOLBAR_SIZE;
        let mut params = params.clone();
        params.color = GRAY;
        let lines = contents.lines().count();
        for i in 1..=lines {
            draw_text_ex(&i.to_string(),
                         x, y,
                         params.clone());
            y += self.vert_gap;
        }
    }

    fn draw_cursor(&mut self, contents: &str, font: &Font) {
        // todo: save the cursors pre clamp location
        // todo: so that the cursor, can jump back to it if it hits a line >= the re clamp size
        let font_option = Option::from(font);
        let ptr_size = measure_text("!",font_option, self.font_size.clone() as u16, 1.0);
        // line breaks before the pointer
        let lines_before: usize = contents[..self.textedit.pointer].chars().filter(|&c| c =='\n').count();
        // length of chars between last line break and the pointer
        // find first \n before pointer
        // measure text from \n to pointer - 1
        let mut line_start = contents[..self.textedit.pointer].rfind('\n').unwrap_or(0);
        if line_start != 0{
            line_start += 1;
        }
        let range: &str = &contents[line_start..self.textedit.pointer];
        let chars_before: f32 = measure_text(range,
                                             font_option,
                                             self.font_size as u16, 1.0).width; // measure_text(char, params.font, self.font_size as u16, 1.0).width
        let ptr_x: f32 = self.indent + TOOLBAR_SIZE + chars_before;
        let ptr_y: f32 = self.font_size * (lines_before + 1) as f32 - ptr_size.offset_y - self.mouse_wheel_y;
        draw_rectangle(ptr_x, ptr_y, // draws starting from what should be bottom.
                       2.0,
                       ptr_size.height,
                       RED);
    }

    fn read_inputs(&mut self) {
        let key = get_last_key_pressed();
        self.key = key;
        self.handle_inputs();
    }
}

impl GlobalInputHandle for EditorGui {
    type GuiType = EditorGui;
    fn key(&self) -> Option<KeyCode> { self.key }
    fn context(&self) -> Textedit { self.textedit.clone() } // I really don't want to do this
    fn gui(&self) -> Self::GuiType { self.clone() } // I really don't want to do this
    fn set_context(&mut self, new_textedit: Textedit) { self.textedit = new_textedit; }
    fn set_gui(&mut self, new_gui: Self::GuiType) { *self = new_gui; }
}

impl Gui for EditorGui {
    fn name(&self) -> &'static str { "Editor" }
}