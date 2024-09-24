/*
 * Goal:
 * 1. Create a simple text editor for C
 * 2. Create a simple compiler for C
 * knowledge here: https://www.geeksforgeeks.org/phases-of-a-compiler/
 */

mod compiler;
mod tests;
mod editor;

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
    let window_height: f32 = 720.;
    let window_width: f32 = 1280.;

    loop {
        draw_text(format!("{}", get_fps()).as_str(), (window_width - 30.), 20., 32., YELLOW);


        /// Clear screen, go next
        next_frame().await
    }
}