/*
 * Goal:
 * 1. Create a simple text editor for our own language
 * 2. Create a simple compiler for our own language
 * knowledge here: https://www.geeksforgeeks.org/phases-of-a-compiler/
 */

mod compiler;
mod tests;
mod editor;
mod types;
mod traits;
mod gui;

use macroquad::prelude::*;
use crate::gui::editor::EditorGui;

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
    let window_height: f32 = 720.;
    let window_width: f32 = 1280.;
    let editor_font = load_ttf_font("src/assets/VictorMono.ttf")
        .await
        .unwrap();
    let mut editor = EditorGui::new(editor_font);
    loop {
        draw_text(format!("{}", get_fps()).as_str(), (window_width - 30.), 20., 32., YELLOW);
        editor.draw();
        // Clear screen, go next
        next_frame().await
    }
    // todo: write changes on editor close.
}