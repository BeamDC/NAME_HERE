use macroquad::prelude::Font;
use crate::gui::editor::EditorGui;
use crate::terminal::terminal::Terminal;
use crate::gui::toolbar::Toolbar;
use crate::traits::gui::Gui;

pub(crate)mod editor;
pub(crate)mod toolbar;
mod terminal;

// todo: make global gui manager, that can switch between guis.
pub struct GuiManager {
    toolbar: Toolbar,
    editor: EditorGui,
    terminal: Terminal,
}

impl GuiManager {
    pub fn new(font: Font) -> Self {
        Self {
            toolbar: Toolbar::new(),
            editor: EditorGui::new(font),
            terminal: Terminal::new(),
        }
    }
}