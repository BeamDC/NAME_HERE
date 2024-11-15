use macroquad::prelude::{load_ttf_font, Font};
use crate::gui::editor::EditorGui;
use crate::gui::terminal::TerminalGui;
use crate::terminal::terminal::Terminal;
use crate::gui::toolbar::{Icons, Toolbar};
use crate::traits::gui::Gui;
use crate::traits::input_handler::GlobalInputHandle;

pub(crate)mod editor;
pub(crate)mod toolbar;
mod terminal;

#[derive(Debug)]
enum PossibleGuis {
    Editor,
    Terminal,
    None,
}

// todo: add ability to check which gui is currently open
pub struct GuiManager {
    toolbar: Toolbar,
    editor: EditorGui,
    terminal: TerminalGui,
    active: PossibleGuis,
}

impl GuiManager {
    pub fn new(font: Font) -> Self {
        Self {
            toolbar: Toolbar::new(),
            editor: EditorGui::new(font.clone()),
            terminal: TerminalGui::new(font.clone()),
            active: PossibleGuis::Editor,
        }
    }

    pub fn init(&mut self) {
        self.editor.textedit.file = Some("src/editor/default.txt".to_owned());
        self.editor.textedit.read().unwrap();
        // add null at EOF, to help out of bounds errors
        self.editor.textedit.buffer.push(0);
        self.terminal.terminal.textedit.buffer.push(0);
    }

    pub fn read_inputs(&mut self) {
        // detect icon clicks
        match self.toolbar.selected {
            Some(Icons::Editor) => { self.active = PossibleGuis::Editor; }
            Some(Icons::Terminal) => { self.active = PossibleGuis::Terminal; }
            _ => {},
        }
    }

    pub fn draw(&mut self) {
        self.read_inputs();
        match self.active {
            PossibleGuis::Editor => { self.editor.draw(); },
            PossibleGuis::Terminal => { self.terminal.draw(); }
            _ => {}
        }
        // println!("{:?}", self.active);
        // draw the toolbar
        self.toolbar.draw();
    }
}