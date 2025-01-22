/*
 * Goal: 🤥
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

use crate::gui::GuiManager;
use macroquad::prelude::*;

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
    let font = load_ttf_font("src/assets/fonts/Hack.ttf")
        .await
        .unwrap();
    let mut app = GuiManager::new(font);
    app.init();

    loop {
        app.draw();
        next_frame().await
    }
    // todo: write changes on editor close.
}