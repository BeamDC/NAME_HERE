/*
 * Goal:
 * 1. Create a simple text editor for our own language
 * 2. Create a simple compiler for our own language
 * knowledge here: https://www.geeksforgeeks.org/phases-of-a-compiler/
 */

// get icons (256x256) here https://fonts.google.com/icons?icon.size=64&icon.color=%23e8eaed

mod compiler;
mod tests;
mod editor;
mod types;
mod gui;
mod terminal;
mod constants;
mod math;

use macroquad::prelude::*;
use crate::gui::editor::EditorGui;
use crate::gui::GuiManager;

fn window_conf() -> Conf {
    Conf {
        window_title: "EDITOR".to_owned(),
        // fullscreen: true,
        window_height: 720,
        window_width: 1280,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = load_ttf_font("src/assets/fonts/VictorMono.ttf")
        .await
        .unwrap();
    let mut app = GuiManager::new(font);
    app.init();

    loop {
        // draw_text(format!("{}", get_fps()).as_str(), screen_width() * 0.75, 20.0, 30.0, YELLOW);
        app.draw();
        next_frame().await
    }
    // todo: write changes on editor close.
}