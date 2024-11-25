use macroquad::color::Color;

pub const TOOLBAR_SIZE: f32 = 50.0;
pub const TOOLBAR_COLOR: Color = Color::new(0.22, 0.22, 0.22, 1.0);

// syntax highlighting
macro_rules! normalize_color { // rbg values are more readable
    ($colors: expr) => {
        Color::new(
            $colors.0 as f32 / 255.0,
            $colors.1 as f32 / 255.0,
            $colors.2 as f32 / 255.0,
            1.0,
        )
    };
}

pub const NUMERIC_COLOR: Color = normalize_color!((29, 79, 171));
pub const KEYWORD_COLOR: Color = normalize_color!((181, 66, 0));
pub const OPERATOR_COLOR: Color = normalize_color!((255, 255, 255));

pub const STRING_COLOR: Color = normalize_color!((4, 77, 27));
pub const IDENT_COLOR: Color = normalize_color!((95, 42, 161));
pub const COMMENT_COLOR: Color = normalize_color!((48, 48, 48));
