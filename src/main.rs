/*
 * Goal:
 * 1. Create a simple text editor for C
 * 2. Create a simple compiler for C
 * knowledge here: https://www.geeksforgeeks.org/phases-of-a-compiler/
 */

mod compiler;
mod web;
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
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}