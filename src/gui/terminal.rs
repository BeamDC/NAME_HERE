use crate::editor::texteditor::Textedit;
use crate::gui::drawing::DrawTextedit;
use crate::gui::gui::Gui;
use crate::gui::input_handler::GlobalInputHandle;
use crate::terminal::terminal::Terminal;
use macroquad::color::WHITE;
use macroquad::input::{get_last_key_pressed, mouse_wheel, KeyCode};
use macroquad::math::clamp;
use macroquad::prelude::{Font, TextParams};

#[derive(Clone)]
pub struct TerminalGui {
    pub terminal: Terminal,
    font_size: f32,
    font: Font,
    indent: f32,
    vert_gap: f32,
    _mouse_wheel_x: f32,
    mouse_wheel_y: f32,
    key: Option<KeyCode>,
}

impl TerminalGui {
    pub fn new(font: Font) -> Self {
        let font_size = 30.0;
        Self {
            terminal: Terminal::new(),
            font_size,
            font,
            indent: font_size,
            vert_gap: 30.0,
            _mouse_wheel_x: 0.0,
            mouse_wheel_y: 0.0,
            key: None,
        }
    }

    pub fn draw(&mut self) {
        let contents  = String::from_utf8(self.terminal.textedit.buffer.clone())
            .unwrap();
        let font = self.font.clone();
        let params: TextParams = TextParams {
            font: Option::from(&font),
            font_size: self.font_size.clone() as u16,
            color: WHITE,
            ..Default::default()
        };

        //
        self.read_inputs();
        self.terminal.get_response();
        println!("{}", self.terminal.current_response.response);
        self.draw_contents(&contents, &params, Some(2.0));
        self.draw_cursor(&contents, &font, Some(2.0));
    }

    fn read_inputs(&mut self) {
        self.mouse_wheel_y -= mouse_wheel().1;
        self.mouse_wheel_y = clamp(self.mouse_wheel_y, 0.0, f32::MAX);
        let key = get_last_key_pressed();
        self.key = key;
        self.handle_inputs();
    }
}

impl Gui for TerminalGui {
    fn name(&self) -> &'static str { "Terminal" }
}

impl GlobalInputHandle for TerminalGui {
    type GuiType = TerminalGui;
    fn key(&self) -> Option<KeyCode> { self.key }
    fn textedit(&self) -> Textedit { self.terminal.textedit.clone() } // I really don't want to do this
    fn gui(&self) -> Self::GuiType { self.clone() } // I really don't want to do this
    fn set_context(&mut self, new_textedit: Textedit) { self.terminal.textedit = new_textedit; }
    fn set_gui(&mut self, new_gui: Self::GuiType) { *self = new_gui; }
}

impl DrawTextedit for TerminalGui {
    fn vert_gap(&self) -> f32 { self.vert_gap }
    fn indent(&self) -> f32 { self.indent }
    fn font_size(&self) -> f32 { self.font_size }
    fn mouse_wheel_y(&self) -> f32 { self.mouse_wheel_y }
    fn textedit(&self) -> &Textedit { &self.terminal.textedit }
}