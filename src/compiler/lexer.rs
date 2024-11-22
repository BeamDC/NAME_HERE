/*
 * read in the buffer contents. split them into tokens
 *
 * Shoutout OLC / Javidx9 for teaching me this
 * Github : https://github.com/OneLoneCoder/Javidx9/blob/master/SimplyCode/OneLoneCoder_DIYLanguage_Tokenizer.cpp
 * Video  : https://www.youtube.com/watch?v=wrj3iuRdA-M
 */

use std::iter::Peekable;
use std::str::{CharIndices, Chars};

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

const WHITESPACE: [bool; 256] = make_lut(" \t\n\r\0");
const INTEGER_DIGITS: [bool; 256] = make_lut("0123456789");
const REAL_DIGITS: [bool; 256] = make_lut(".0123456789");
const OPERATORS: [bool; 256] = make_lut(r"!$%^&*+-=#@?|`/\<>~");
const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_1234567890"
);
const KEYWORDS: [&str; 8] = [
    "return", "if", "else", "for", "while", "bool", "false", "true"
];

pub enum Token {
    Unknown(Vec<char>),
    Keyword(Vec<char>),
    Ident(Vec<char>),
    String(Vec<char>),
    Operator(Vec<char>),
    Separator(Vec<char>),
    Numeric(Vec<char>),
    Lparen(char),
    Rparen(char),
    Lsquare(char),
    Rsquare(char),
    Lcurly(char),
    Rcurly(char),
    Langled(char),
    Rangled(char),
    Whitespace(char),
    EOF(char),
}

pub struct Lexer<'a> {
    pub src: &'a str,
    chars: Peekable<Chars<'a>>, // might only need .chars(), re evaluate when complete
    pub tokens: Vec<Token>
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: "",
            chars: src.chars().peekable(),
            tokens: vec![],
        }
    }

    pub fn parse(&mut self) {
        enum State {
            New,
            String,
            Numeric,
            Operator,
            Separator,
            Name,
            EndOfLine,
            Lparen,
            Rparen,
            Lsquare,
            Rsquare,
            Lcurly,
            Rcurly,
            Langled,
            Rangled,
            Complete,
        }

        let current_state: State = State::New;
        let mut next_state: State = State::New;

        let mut current_value: Vec<char> = vec![];
        let mut current_token: Token = Token::Unknown(vec![]);

        let mut decimal_found: bool = false;
        let paren_balance: usize = 0;
        let square_balance: usize = 0;
        let curly_balance: usize = 0;
        let angled_balance: usize = 0;

        let mut current_char: char = self.chars.next().unwrap_or( '\0' );
        while (current_char != '\0') {
            match current_state {
                State::New => { // Determine the type of the token to be created
                    current_value.clear();
                    current_token = Token::Unknown(vec![]);
                    decimal_found = false;

                    // for now, if whitespace is found, consume it
                    if WHITESPACE[current_char] {
                        current_char = self.chars.next().unwrap_or( '\0' );
                        next_state = State::New;
                    }
                    else if REAL_DIGITS[current_char] {
                        // base 10, will later add a case for non-decimal numbers
                        next_state = State::Numeric;
                    }
                    else if OPERATORS[current_char] {
                        next_state = State::Operator;
                    }
                    else {
                        match current_char {
                            '(' => next_state = State::Lparen,
                            ')' => next_state = State::Rparen,
                            '[' => next_state = State::Lsquare,
                            ']' => next_state = State::Rsquare,
                            '{' => next_state = State::Lcurly,
                            '}' => next_state = State::Rcurly,
                            '<' => next_state = State::Langled,
                            '>' => next_state = State::Rangled,
                            ',' => next_state = State::Separator,
                            ';' => next_state = State::EndOfLine,
                            '"' => next_state = State::String,
                            _ => next_state = State::Name,
                        }
                    }
                }
                State::String => {},
                State::Numeric => {},
                _ => {}
            }
        }
    }
}