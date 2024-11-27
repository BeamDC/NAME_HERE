/*
 * read in the buffer contents. split them into tokens
 *
 * Shoutout OLC / Javidx9 for teaching me this
 * Github : https://github.com/OneLoneCoder/Javidx9/blob/master/SimplyCode/OneLoneCoder_DIYLanguage_Tokenizer.cpp
 * Video  : https://www.youtube.com/watch?v=wrj3iuRdA-M
 *
 * This will have to be cleaned up, i cant handle the nesting
 */
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::str::Chars;
use crate::compiler::operators::{Operator, OperatorMap};

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
const OPERATORS: [bool; 256] = make_lut(r"!$%^&*+-=#@|`/\<>~.");
const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_1234567890"
);
const KEYWORDS: [&str; 22] = [
    "let",
    "if", "else", "for", "while",
    "bool", "false", "true",
    "i8", "i16", "i32", "i64",
    "u8", "u16", "u32", "u64",
    "f32", "f64",
    "char", "str",
    "fn", "return",
];

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    Unknown,
    Keyword,
    Function,
    Ident,
    String,
    Operator,
    Numeric,
    Separator,
    EndOfLine,
    Comment,
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lcurly,
    Rcurly,
    Langled,
    Rangled,
    Whitespace,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            value ,
        }
    }
}


pub struct Lexer<'a> {
    pub src: &'a str,
    chars: Chars<'a>,
    pub tokens: Vec<Token>,
    operators: HashMap<String, Operator>,
}

// todo: allow the lexer to have its source changed, make parse return the tokens instead of storing them
impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: "",
            chars: src.chars(),
            tokens: vec![],
            operators: OperatorMap::new().operators,
        }
    }

    pub fn tokenize(&mut self) {
        #[derive(Clone, Copy, PartialEq, Debug)]
        enum State {
            New,
            WhiteSpace,
            String,
            Numeric,
            Operator,
            Separator,
            Name,
            Comment,
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
        let mut current_token: Token = Token::new(TokenType::Unknown, String::new());

        let mut decimal_found: bool = false;
        let mut paren_balance: isize = 0;
        let mut square_balance: isize = 0;
        let mut curly_balance: isize = 0;
        let mut angled_balance: isize = 0;

        let mut current_char: char = self.chars.next().unwrap_or( '\0' );
        while current_char != '\0' || current_state != State::New{
            match current_state {
                State::New => { // Determine the type of the token to be created
                    current_value.clear();
                    current_token = Token::new(TokenType::Unknown, String::new());
                    decimal_found = false;

                    if WHITESPACE[current_char as usize] {
                        next_state = State::WhiteSpace;
                    }
                    // we dont allow a '.' to start a num
                    else if INTEGER_DIGITS[current_char as usize] {
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
                            '?' => next_state = State::Comment,
                            '"' => {
                                current_char = self.chars.next().unwrap_or( '\0' );
                                next_state = State::String;
                            },
                            _ => {
                                next_state = State::Name
                            },
                        }
                    }
                }
                State::WhiteSpace => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Whitespace, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    // prev_state = State::WhiteSpace;
                    next_state = State::Complete;
                },
                State::String => {
                    // we have to watch for the case where the string is not closed
                    if current_char == '"' || current_char == '\0' {
                        current_char = self.chars.next().unwrap_or( '\0' );
                        // current_value.push(current_char);
                        current_token = Token::new(TokenType::String, current_value.clone());
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
                                panic!("Unexpected character in numeric value : {:?}", current_char);
                            }
                            else { decimal_found = true; }
                        }
                        current_value.push(current_char);
                        current_char = self.chars.next().unwrap_or( '\0' );
                    }else {
                        if IDENT_CHARS[current_char as usize] {
                            panic!("Unexpected character in numeric value : {:?}", current_char);
                        }
                        else {
                            current_token = Token::new(TokenType::Numeric, current_value.clone());
                            next_state = State::Complete;
                        }
                    }

                },
                State::Operator => {
                    if OPERATORS[current_char as usize] {
                        current_value.push(current_char);
                        println!("{:?}", current_value);
                        if self.operators.contains_key(&current_value) {
                            current_char = self.chars.next().unwrap_or( '\0' );
                        }
                        else {
                            current_value.pop();
                            if self.operators.contains_key(&current_value) {
                                current_token = Token::new(TokenType::Operator, current_value.clone());
                                next_state = State::Complete;
                            }
                            else {
                                current_value.push(current_char);
                                current_char = self.chars.next().unwrap_or( '\0' );
                            }
                        }
                    }else {
                        let valid_prev = !(self.tokens_filter_whitespace()
                            .last()
                            .unwrap_or(&Token::new(TokenType::Unknown, String::new()))
                            .token_type ==
                            TokenType::Numeric);

                        let valid_unary =
                            (current_value == "-" || current_value == "+") && valid_prev;

                        if self.operators.contains_key(&current_value) && valid_unary {
                            current_value.insert(0, 'u');
                            current_token = Token::new(TokenType::Operator, current_value.clone());
                            next_state = State::Complete;
                        }
                        else if self.operators.contains_key(&current_value) {
                            current_token = Token::new(TokenType::Operator, current_value.clone());
                            next_state = State::Complete;
                        }
                        else {
                            panic!("Unexpected operator {:?}", current_value);
                        }
                    }
                },
                State::Lparen => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Lparen, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance += 1;
                    next_state = State::Complete;
                },
                State::Rparen => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Rparen, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance -= 1;
                    next_state = State::Complete;
                },
                State::Lsquare => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Lsquare, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance += 1;
                    next_state = State::Complete;
                },
                State::Rsquare => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Rsquare, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance -= 1;
                    next_state = State::Complete;
                },
                State::Lcurly => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Lcurly, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance += 1;
                    next_state = State::Complete;
                },
                State::Rcurly => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Rcurly, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance -= 1;
                    next_state = State::Complete;
                },
                State::Langled => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Langled, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance += 1;
                    next_state = State::Complete;
                },
                State::Rangled => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Rangled, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance -= 1;
                    next_state = State::Complete;
                },
                State::Separator => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::Separator, current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    next_state = State::Complete;
                },
                State::EndOfLine => {
                    current_value.push(current_char);
                    current_token = Token::new(TokenType::EndOfLine, current_value.clone());
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
                            current_token = Token::new(TokenType::Keyword, current_value.clone());
                        }
                        else if self.tokens_filter_whitespace()
                            .last()
                            .unwrap_or(&Token::new(TokenType::Unknown, String::new()))
                            .value == "fn" {
                            current_token = Token::new(TokenType::Function, current_value.clone());
                        }
                        else {
                            current_token = Token::new(TokenType::Ident, current_value.clone());
                        }
                        next_state = State::Complete;
                    }
                },
                State::Comment => {
                    if current_char == '\n' || current_char == '\0' {
                        current_token = Token::new(TokenType::Comment, current_value.clone());
                        next_state = State::Complete;
                    }
                    else {
                        current_value.push(current_char);
                        current_char = self.chars.next().unwrap_or( '\0' );
                    }
                },
                State::Complete =>  {
                    self.tokens.push(current_token.clone());
                    next_state = State::New;
                },
            }
            current_state = next_state;
        }
    }
    pub fn tokens_filter_whitespace(&self) -> Vec<Token> {
        let mut filtered_tokens: Vec<Token> = vec![];
        for token in &self.tokens {
            if token.token_type != TokenType::Whitespace {
                filtered_tokens.push(token.clone());
            }
        }
        filtered_tokens
    }
}