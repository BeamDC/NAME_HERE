use macroquad::input::{get_last_key_pressed, KeyCode};
use macroquad::prelude::Font;
use crate::editor::texteditor::Textedit;
use crate::terminal::terminal::Terminal;
use crate::traits::gui::Gui;
use crate::traits::input_handler::GlobalInputHandle;

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
            indent: font_size * 3.0,
            vert_gap: 30.0,
            _mouse_wheel_x: 0.0,
            mouse_wheel_y: 0.0,
            key: None,
        }
    }

    pub fn draw(&mut self) { // todo: make draw_contents a trait and apply it to the terminal
        self.read_inputs();
    }

    fn read_inputs(&mut self) {
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
    fn context(&self) -> Textedit { self.terminal.textedit.clone() } // I really don't want to do this
    fn gui(&self) -> Self::GuiType { self.clone() } // I really don't want to do this
    fn set_context(&mut self, new_textedit: Textedit ) { self.terminal.textedit = new_textedit; }
    fn set_gui(&mut self, new_gui: Self::GuiType) { *self = new_gui; }
}