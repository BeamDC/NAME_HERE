// im so sorry this is unreadable.

use macroquad::color::Color;
use macroquad::input::mouse_position;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, draw_texture_ex, Rect};
use macroquad::prelude::Texture2D;
use macroquad::prelude::WHITE;
use macroquad::prelude::{draw_rectangle_ex, draw_rectangle_lines, screen_height, DrawRectangleParams};
use macroquad::texture::DrawTextureParams;

macro_rules! render_icon {
    ($path: expr, $x: expr, $y: expr, $params: expr) => {
        draw_texture_ex(
            &Texture2D::from_file_with_format(include_bytes!($path), None),
            $x,
            $y,
            WHITE,
            $params.clone(),
        );
    };
}

macro_rules! indicate_hovered {
    ($hovered: expr, $current: expr, $position: expr, $size: expr) => {
        if let Some(hovered) = $hovered{ // this code is horrible, but it works so cry about it
            if hovered == $current {
                draw_rectangle(
                    $position.x, $position.y,
                    $size, $size,
                    Color::new(0.28, 0.28, 0.28, 1.0),
                );
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Icons {
    FileOpen,
    Search,
    Editor,
    Terminal,
    Settings,
    Icon,
}

#[derive(Clone)]
pub struct Toolbar {
    pub width: f32,
    pub icon_size: f32,
    pub bg_params: DrawRectangleParams,
    pub(crate) icons: Vec<(Icons, Vec2)>, // (icon type, position)
    pub hovered: Option<Icons>,
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
            icons: vec![
                (Icons::FileOpen, Vec2::ZERO),
                (Icons::Search, Vec2::ZERO),
                (Icons::Editor, Vec2::ZERO),
                (Icons::Terminal, Vec2::ZERO),
                (Icons::Settings, Vec2::ZERO),
                (Icons::Icon, Vec2::ZERO),
            ],
            hovered: None,
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle_ex(0.0, 0.0, self.width, screen_height(), self.bg_params.clone());
        self.draw_icons(true);
    }

    pub fn draw_icons(&mut self, bounding: bool) {
        let padding = (self.width - self.icon_size) / 2.0;
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.icon_size, self.icon_size)),
            ..Default::default()
        };

        // check if the mouse is over an icon
        let (mouse_x,mouse_y) = mouse_position();
        self.hovered = self.icons
            .iter()
            .find(|(_, pos)| {
                let bounds = Rect::new(
                    pos.x,
                    pos.y,
                    self.icon_size,
                    self.icon_size,
                );
                bounds.contains(Vec2::new(mouse_x,mouse_y))
            })
            .map(|(icon, _)| *icon);

        for (i, (icon, position)) in self.icons.iter_mut().enumerate() {
            let y_position = (i as f32 + 1.0) * self.width / 5.0 + (i as f32) * self.icon_size;
            *position = Vec2::new(padding, y_position);

            // why must include_bytes! require a string literal T_T
            match icon {
                Icons::FileOpen => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/file_open.png", position.x, position.y, &params);
                }
                Icons::Editor => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/editor.png", position.x, position.y, &params);
                }
                Icons::Terminal => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/terminal.png", position.x, position.y, &params);
                }
                Icons::Search => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/search.png", position.x, position.y, &params);
                }
                Icons::Settings => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/settings.png", position.x, position.y, &params);
                }
                Icons::Icon => {
                    indicate_hovered!(self.hovered, *icon, position, self.icon_size);
                    render_icon!("../../src/assets/icons/icon.png", position.x, position.y, &params);
                }
            }

            // Draw bounding box if enabled
            if bounding {
                draw_rectangle_lines(
                    position.x,
                    position.y,
                    self.icon_size,
                    self.icon_size,
                    2.0,
                    Color::from_rgba(255,0,0,127),
                );
            }
        }
    }
}