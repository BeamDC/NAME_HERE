use macroquad::color::Color;

pub const TOOLBAR_SIZE: f32 = 50.0;
pub const TOOLBAR_COLOR: Color = Color::new(0.22, 0.22, 0.22, 1.0);

// syntax highlighting
macro_rules! normalize_color { // rgb values are more readable
    ($colors: expr) => {
        Color::new(
            $colors.0 as f32 / 255.0,
            $colors.1 as f32 / 255.0,
            $colors.2 as f32 / 255.0,
            1.0,
        )
    };
}

pub const NUMERIC_COLOR: Color = normalize_color!((2, 153, 130));
pub const KEYWORD_COLOR: Color = normalize_color!((181, 66, 0));
pub const OPERATOR_COLOR: Color = normalize_color!((255, 255, 255));
pub const STRING_COLOR: Color = normalize_color!((4, 77, 27));
pub const FUNCTION_COLOR: Color = normalize_color!((2, 135, 237));
pub const IDENT_COLOR: Color = normalize_color!((255, 255, 255));
pub const COMMENT_COLOR: Color = normalize_color!((64, 64, 64));

// Lookup Tables & Keywords for Lexer
const fn make_lut(chars: &str) -> [bool; 256] {
    let mut lut = [false; 256];
    let bytes = chars.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        lut[bytes[i] as usize] = true;
        i += 1;
    }
    lut
}

pub const WHITESPACE: [bool; 256] = make_lut(" \t\n\r\0");
pub const INTEGER_DIGITS: [bool; 256] = make_lut("0123456789");
pub const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"
);
pub const KEYWORDS: [&str; 13] = [
    "fn",
    "bool", "true", "false",
    "let", "const",
    "if", "else",
    "for", "while", "loop",
    "nil",
    "return"
];
