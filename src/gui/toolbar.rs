use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_texture_ex;
use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};
use macroquad::texture::{DrawTextureParams, Texture2D};
use macroquad::window::screen_height;

macro_rules! render_icon {
    ($path: expr,$x: expr,$y: expr, $params: expr) => {
        draw_texture_ex(
            &Texture2D::from_file_with_format(
                include_bytes!($path),
                None,
            ),
            $x, $y,
            WHITE,
            $params.clone(),
        );
        $y += 10.0 + $params.dest_size.unwrap().y;
    };
}

// for the toolbar we will eventually add to the editor.
pub struct Toolbar {
    pub width: f32,
    pub bg_params: DrawRectangleParams,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            width: 50.0,
            bg_params: DrawRectangleParams {
                offset: Vec2::new(0.0, 0.0),
                rotation: 0.0,
                color: Color::new(0.22, 0.22, 0.22, 1.0),
            },
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle_ex(0.0,0.0,self.width,screen_height(), self.bg_params.clone());
        self.draw_icons();
    }
    pub fn draw_icons(&mut self) {
        /*
         * Render icons to the screen as textures
         */
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(34.0,34.0)),
            ..Default::default()
        };
        let mut y: f32 = 10.0;
        render_icon!("../../src/assets/icons/file_open.png", 8.0, y, &params);
        render_icon!("../../src/assets/icons/search.png", 8.0, y, &params);
        render_icon!("../../src/assets/icons/settings.png", 8.0, y, &params);
        render_icon!("../../src/assets/icons/icon.png", 8.0, y, &params);
    }
}