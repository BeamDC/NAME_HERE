/*
 * Goal:
 * Create a basic programming language
 * Compile said language targeting wasm
 * solve some online problems in my language :)
 * knowledge here: https://www.geeksforgeeks.org/phases-of-a-compiler/
 */

mod compiler;
mod web;
mod tests;
mod editor;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}