use macroquad::prelude::Font;

pub struct SettingsGui {
    font_size: f32,
    font: Font,
    _mouse_wheel_x: f32,
    mouse_wheel_y: f32,
}

impl SettingsGui {
    pub fn new(font: Font) -> Self {
        let font_size = 30.0;
        Self {
            font_size,
            font,
            _mouse_wheel_x: 0.0,
            mouse_wheel_y: 0.0,
        }
    }
}