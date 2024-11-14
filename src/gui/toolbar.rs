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
    pub icon_size: f32,
    pub bg_params: DrawRectangleParams,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            width: 50.0,
            icon_size: 34.0,
            bg_params: DrawRectangleParams {
                offset: Vec2::new(0.0, 0.0),
                rotation: 0.0,
                color: Color::new(0.22, 0.22, 0.22, 1.0),
            },
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle_ex(0.0,0.0,self.width,screen_height(), self.bg_params.clone());
        self.draw_icons(true);
    }
    pub fn draw_icons(&mut self, bounding: bool) {
        let padding = (self.width - self.icon_size) / 2.0;
        let mut icon_gap = self.width / 5.0;
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.icon_size,self.icon_size)),
            ..Default::default()
        };

        render_icon!("../../src/assets/icons/file_open.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + self.icon_size;
        render_icon!("../../src/assets/icons/editor.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + self.icon_size;
        render_icon!("../../src/assets/icons/search.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + self.icon_size;
        render_icon!("../../src/assets/icons/settings.png", padding, icon_gap, &params);
        icon_gap += self.width / 5.0 + self.icon_size;
        render_icon!("../../src/assets/icons/icon.png", padding, icon_gap, &params);

        // draw bounding box of each icon
        if bounding {
            todo!()
        }
    }
}