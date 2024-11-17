use crate::editor::texteditor::Textedit;
use crate::traits::drawing::DrawTextedit;
use crate::traits::gui::Gui;
use crate::traits::input_handler::GlobalInputHandle;
use macroquad::color::WHITE;
use macroquad::input::{get_last_key_pressed, mouse_wheel, KeyCode};
use macroquad::math::clamp;
use macroquad::text::{Font, TextParams};

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

impl DrawTextedit for EditorGui {
    fn vert_gap(&self) -> f32 { self.vert_gap }
    fn indent(&self) -> f32 { self.indent }
    fn font_size(&self) -> f32 { self.font_size }
    fn mouse_wheel_y(&self) -> f32 { self.mouse_wheel_y }
    fn textedit(&self) -> &Textedit { &self.textedit }
}