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
    };
}

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
        let icon_size = 34.0;
        let padding = (self.width - icon_size) / 2.0;
        let mut icon_gap = self.width / 5.0;
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(icon_size,icon_size)),
            ..Default::default()
        };

        render_icon!("../../src/assets/icons/file_open.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + icon_size;
        render_icon!("../../src/assets/icons/search.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + icon_size;
        render_icon!("../../src/assets/icons/settings.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + icon_size;
        render_icon!("../../src/assets/icons/icon.png", padding, icon_gap, &params);

        // todo: we need to know when the user clicks an icon.
        // We can split the toolbar into a grid where each square
        // is the size and position of an icon.
        // depending on which square is clicked, do smth
    }
}