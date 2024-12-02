use std::fs;
use macroquad::prelude::Font;
use crate::gui::editor::EditorGui;
use crate::gui::terminal::TerminalGui;
use crate::gui::toolbar::{Icons, Toolbar};
use rfd::FileDialog;
// use gui::Gui;
// use input_handler::GlobalInputHandle;

pub(crate)mod editor;
pub(crate)mod toolbar;
mod terminal;
pub mod input_handler;
pub mod gui;
pub mod drawing;

#[derive(Debug)]
enum PossibleGuis {
    Editor,
    Terminal,
}

pub struct GuiManager {
    toolbar: Toolbar,
    editor: EditorGui,
    terminal: TerminalGui,
    active: PossibleGuis,
    previous: Icons,
}

impl GuiManager {
    pub fn new(font: Font) -> Self {
        Self {
            toolbar: Toolbar::new(font.clone()),
            editor: EditorGui::new(font.clone()),
            terminal: TerminalGui::new(font.clone()),
            active: PossibleGuis::Editor,
            previous: Icons::Editor,
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
            Some(Icons::Editor) => {
                self.active = PossibleGuis::Editor;
                self.previous = self.toolbar.selected.unwrap();
            }
            Some(Icons::Terminal) => {
                self.active = PossibleGuis::Terminal;
                self.previous = self.toolbar.selected.unwrap();
            }
            // File Open Business
            Some(Icons::FileOpen) => {
                // todo: this is different for other OS, so well have to add a check for the users system
                let file = FileDialog::new()
                    .set_title("Choose your file wisely...")
                    .add_filter("text", &["txt"])
                    .add_filter("rust", &["rs", "toml"])
                    .set_directory("/")
                    .pick_file();

                if file.is_some() {
                    let filepath = file.unwrap().to_str().unwrap().to_owned();
                    self.editor.textedit.write().unwrap();
                    self.editor.textedit.file = Some(filepath);
                    self.editor.textedit.read().unwrap();
                    self.editor.textedit.buffer.push(0);
                    self.editor.textedit.redraw = true;

                    //let content = fs::read_to_string(file.unwrap_or_default()).expect("No path specified in file open (gui/mod.rs)");
                    //println!("{}", content);
                }
                self.toolbar.selected = Option::from(self.previous);
            }
            _ => {},
        }
    }

    pub fn draw(&mut self) {
        self.read_inputs();
        match self.active {
            PossibleGuis::Editor => { self.editor.draw(); },
            PossibleGuis::Terminal => { self.terminal.draw(); }
        }
        // println!("{:?}", self.active);
        // draw the toolbar
        self.toolbar.draw();
    }
}