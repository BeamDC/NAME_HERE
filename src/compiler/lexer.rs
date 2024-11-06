/*
 * read in the buffer contents. split them into tokens
 * knowledge: https://mohitkarekar.com/posts/pl/lexer/
 *
 * References: ( stuff I don't really get atm)
 * https://doc.rust-lang.org/std/iter/struct.Peekable.html
 * https://doc.rust-lang.org/std/str/struct.CharIndices.html
 */

use std::iter::Peekable;
use std::str::CharIndices;

// Lookup tables
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

const WHITESPACE: [bool; 256] = make_lut(" \t\n\r");
const INTEGER_DIGITS: [bool; 256] = make_lut("0123456789");
const REAL_DIGITS: [bool; 256] = make_lut(".0123456789");
const OPERATORS: [bool; 256] = make_lut("$%^&*+-=#@?|`/\\<>~");
const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_1234567890"
);
const KEYWORDS: [&str; 8] = [
    "return","if","else","for","while","bool","false","true"
];

pub enum Token {
    Keyword(Vec<char>),
    Ident(Vec<char>),

    // types
    String(Vec<char>),

    // operators
    Plus(char),
    Minus(char),
    Multiply(char),
    Divide(char),
    Exponential(char),

    // parenthesis variations
    Lparen(char),
    Rparen(char),
    Lsquare(char),
    Rsquare(char),
    Lcurly(char),
    Rcurly(char),
    Langled(char),
    Rangled(char),

    // control
    Whitespace(char),
    EOF(char),
}

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<CharIndices<'a>>,
    current: Option<(usize, char)>,
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: "",
            chars: src.char_indices().peekable(),
            current: None,
        }
    }

    pub fn get_next(&mut self) -> Token{
        self.current = self.chars.next();
        let tok: Token;
        // peek at next, if it's not whitespace, start reading multiline
        // if it is whitespace, match and return.
        // if first char is number, looking at numeric literal
        // if it's a quote it's a string, read until next quote or eof

        match self.current.unwrap_or((0usize, '\0')) { // (usize, char) is the char and its position
            (_,'+') => {tok = Token::Plus('+')}
            (_,'-') => {tok = Token::Minus('-')}
            (_,'*') => {tok = Token::Multiply('*')}
            (_,'/') => {tok = Token::Divide('/')}
            (_,'^') => {tok = Token::Exponential('^')}

            (_,'(') => {tok = Token::Lparen('(')}
            (_,')') => {tok = Token::Rparen(')')}
            (_,'[') => {tok = Token::Lsquare('[')}
            (_,']') => {tok = Token::Rsquare(']')}
            (_,'<') => {tok = Token::Langled('<')}
            (_,'>') => {tok = Token::Rangled('>')}

            (_,' ') => {tok = Token::Whitespace(' ')}
            _ => {tok = Token::EOF('\0')},
        }
        tok
    }
}

pub fn tokenize(contents: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    tokens
}