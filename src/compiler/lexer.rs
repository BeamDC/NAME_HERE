/*
 * read in the buffer contents. split them into tokens
 *
 * Shoutout OLC / Javidx9 for teaching me this
 * Github : https://github.com/OneLoneCoder/Javidx9/blob/master/SimplyCode/OneLoneCoder_DIYLanguage_Tokenizer.cpp
 * Video  : https://www.youtube.com/watch?v=wrj3iuRdA-M
 *
 * This will have to be cleaned up, i cant handle the nesting
 */
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

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

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Unknown(String),
    Keyword(String),
    Ident(String),
    String(String),
    Operator(String),
    Separator(String),
    Numeric(String),
    EndOfLine(String),
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
    pub tokens: Vec<Token>,
    operators: HashMap<String, (usize, usize)>,
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        let mut operators: HashMap<String, (usize, usize)> = HashMap::new();
        // binary operators
        operators.insert("+".to_string(), (1, 2));
        operators.insert("-".to_string(), (1, 2));
        operators.insert("*".to_string(), (3, 2));
        operators.insert("/".to_string(), (3, 2));
        // unary operators
        operators.insert("u+".to_string(), (100, 1));
        operators.insert("u-".to_string(), (100, 1));

        Lexer {
            src: "",
            chars: src.chars().peekable(),
            tokens: vec![],
            operators,
        }
    }

    pub fn parse(&mut self) {
        #[derive(Clone, Copy)]
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

        let mut current_state: State = State::New;
        let mut next_state: State = State::New;

        let mut current_value: String = String::new();
        let mut current_token: Token = Token::Unknown(String::new());

        let mut decimal_found: bool = false;
        let mut paren_balance: isize = 0;
        let mut square_balance: isize = 0;
        let mut curly_balance: isize = 0;
        let mut angled_balance: isize = 0;

        let mut current_char: char = self.chars.next().unwrap_or( '\0' );
        while current_char != '\0' { // todo: add bool, so that when eof is hit, we run through one more time to finish any existing tokens
            match current_state {
                State::New => { // Determine the type of the token to be created
                    current_value.clear();
                    current_token = Token::Unknown(String::new());
                    decimal_found = false;

                    // for now, if whitespace is found, consume it
                    if WHITESPACE[current_char as usize] {
                        current_char = self.chars.next().unwrap_or( '\0' );
                        next_state = State::New;
                    }
                    else if REAL_DIGITS[current_char as usize] {
                        // base 10, will later add a case for non-decimal numbers
                        next_state = State::Numeric;
                        // current_char = self.chars.next().unwrap_or( '\0' );
                    }
                    else if OPERATORS[current_char as usize] {
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
                            '"' => {
                                current_char = self.chars.next().unwrap_or( '\0' );
                                next_state = State::String;
                            },
                            _ => {
                                current_value.push(current_char);
                                next_state = State::Name
                            },
                        }
                    }
                }
                State::String => {
                    if current_char == '"' {
                        current_char = self.chars.next().unwrap_or( '\0' );
                        current_value.push(current_char);
                        current_token = Token::String(current_value.clone());
                        next_state = State::Complete;
                    }
                    else {
                        current_value.push(current_char);
                        current_char = self.chars.next().unwrap_or( '\0' );
                    }
                },
                State::Numeric => {
                    if REAL_DIGITS[current_char as usize] {
                        if current_char == '.' {
                            if decimal_found {
                                panic!("Unexpected character in numeric value");
                            }
                            else { decimal_found = true; }
                        }
                        current_value.push(current_char);
                        current_char = self.chars.next().unwrap_or( '\0' );
                    }else {
                        if IDENT_CHARS[current_char as usize] {
                            panic!("Unexpected character in numeric value");
                        }
                        else {
                            current_token = Token::Numeric(current_value.clone());
                            next_state = State::Complete;
                        }
                    }

                },
                State::Operator => {
                    if OPERATORS[current_char as usize] {
                        current_value.push(current_char);
                        if self.operators.contains_key(&current_value) {
                            current_value.push(current_char);
                            current_char = self.chars.next().unwrap_or( '\0' );
                        }
                        else {
                            current_value.pop();
                            if self.operators.contains_key(&current_value) {
                                current_token = Token::Operator(current_value.clone());
                                next_state = State::Complete;
                            }
                            else {
                                current_value.push(current_char);
                                current_char = self.chars.next().unwrap_or( '\0' );
                            }
                        }
                    }else {
                        if self.operators.contains_key(&current_value) {
                            current_token = Token::Operator(current_value.clone());
                            next_state = State::Complete;
                        }
                        else {
                            panic!("Unexpected operator");
                        }
                    }
                },
                State::Lparen => {
                    current_value.push(current_char);
                    current_token = Token::Lparen(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance += 1;
                    next_state = State::Complete;
                },
                State::Rparen => {
                    current_value.push(current_char);
                    current_token = Token::Rparen(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance -= 1;
                    next_state = State::Complete;
                },
                State::Lsquare => {
                    current_value.push(current_char);
                    current_token = Token::Lsquare(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance += 1;
                    next_state = State::Complete;
                },
                State::Rsquare => {
                    current_value.push(current_char);
                    current_token = Token::Rsquare(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance -= 1;
                    next_state = State::Complete;
                },
                State::Lcurly => {
                    current_value.push(current_char);
                    current_token = Token::Lcurly(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance += 1;
                    next_state = State::Complete;
                },
                State::Rcurly => {
                    current_value.push(current_char);
                    current_token = Token::Rcurly(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance -= 1;
                    next_state = State::Complete;
                },
                State::Langled => {
                    current_value.push(current_char);
                    current_token = Token::Langled(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance += 1;
                    next_state = State::Complete;
                },
                State::Rangled => {
                    current_value.push(current_char);
                    current_token = Token::Rangled(current_char);
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance -= 1;
                    next_state = State::Complete;
                },
                State::Separator => {
                    current_value.push(current_char);
                    current_token = Token::Separator(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    next_state = State::Complete;
                },
                State::EndOfLine => {
                    current_value.push(current_char);
                    current_token = Token::EndOfLine(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    next_state = State::Complete;
                },
                State::Name => {
                    if IDENT_CHARS[current_char as usize] {
                        current_value.push(current_char);
                        current_char = self.chars.next().unwrap_or( '\0' );
                    }
                    else {
                        if KEYWORDS.contains(&current_value.as_str()) {
                            current_token = Token::Keyword(current_value.clone());
                        }
                        else {
                            current_token = Token::Ident(current_value.clone());
                        }
                        next_state = State::Complete;
                    }
                },
                State::Complete => {
                    self.tokens.push(current_token.clone());
                    next_state = State::New;
                },
                _ => {}
            }
            current_state = next_state;
        }

        // on exit, check if a token was still being accumulated,
        // if so, add it
        // if !current_value.is_empty() {
        //     self.tokens.push(current_token.clone());
        // }
    }
}