/*
 * read in the buffer contents. split them into tokens
 *
 * Shoutout Robert Nystrom / Diego Freijo for the inspiration
 * repo -
 * book -
 */
use crate::vm::operators::{Operator, OperatorMap};
use crate::vm::token::{Token, TokenType};
use itertools::{peek_nth, PeekNth};
use std::collections::HashMap;
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
const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_"
);

pub struct Lexer<'a> {
    pub src: &'a str,
    chars: PeekNth<Chars<'a>>,
    pub tokens: Vec<Token>,
    operators: HashMap<String, Operator>,
    start: usize,
    current: usize,
    line: usize,
}

// todo: allow the lexer to have its source changed, make parse return the tokens instead of storing them
impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src,
            chars     : peek_nth(src.chars()),
            tokens    : vec![],
            operators : OperatorMap::new().operators,
            start     : 0,
            current   : 0,
            line      : 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_eof() {
            tokens.push(self.next_token());
        }
        tokens
    }

    pub fn tokenize_no_whitespace(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while !self.is_eof() {
            let token = self.next_token();
            if token.token_type != TokenType::Whitespace {
                tokens.push(token);
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        self.start = self.current;
        match self.advance() {
            Some(c) => match c {
                _ if Lexer::is_digit(c) => self.number(),
                _ if Lexer::is_alpha(c) => self.ident(),
                _ if Lexer::is_whitespace(c) => self.whitespace(),

                // match single chars
                '(' => self.make_token(TokenType::Lparen),
                ')' => self.make_token(TokenType::Rparen),
                '{' => self.make_token(TokenType::Lcurly),
                '}' => self.make_token(TokenType::Rcurly),
                '[' => self.make_token(TokenType::Lsquare),
                ']' => self.make_token(TokenType::Rsquare),
                ';' => self.make_token(TokenType::Semicolon),
                '.' => self.make_token(TokenType::Dot),
                ',' => self.make_token(TokenType::Comma),
                '+' => self.make_token(TokenType::Add),
                '-' => self.make_token(TokenType::Sub),
                '*' => self.make_token(TokenType::Mul),
                '/' => self.make_token(TokenType::Div),

                // match two chars
                // todo : make match that handles multiple possibilities
                '!' => self.match_token(&'=', TokenType::Neq, TokenType::Bang),
                '=' => self.match_token(&'=', TokenType::Equal, TokenType::Assign),
                '<' => self.match_token(&'=', TokenType::Leq, TokenType::Less),
                '>' => self.match_token(&'=', TokenType::Geq, TokenType::Greater),

                // string
                '"' => self.string(),

                // comment
                '?' => self.comment(),

                // error
                _ => self.error(format!("Unexpected char '{}'", c)),
            }
            None => self.eof(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn is_eof(&mut self) -> bool {
        self.peek() == None
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            String::from(&self.src[self.start..self.current])
        )
    }

    fn match_token(
        &mut self,
        expected: &char,
        on_match: TokenType,
        otherwise: TokenType) -> Token {
        if self.matches(expected) {
            self.make_token(on_match)
        }else {
            self.make_token(otherwise)
        }
    }

    fn make_ident(&self) -> Token {
        let value = &self.src[self.start..self.current];
        match value {
            "fn" => self.make_token(TokenType::Fn),
            "bool" => self.make_token(TokenType::Bool),
            "true" => self.make_token(TokenType::True),
            "false" => self.make_token(TokenType::False),
            "let" => self.make_token(TokenType::Let),
            "const" => self.make_token(TokenType::Const),
            "if" => self.make_token(TokenType::If),
            "else" => self.make_token(TokenType::Else),
            "for" => self.make_token(TokenType::For),
            "while" => self.make_token(TokenType::While),
            "loop" => self.make_token(TokenType::Loop),
            "nil" => self.make_token(TokenType::Nil),
            "return" => self.make_token(TokenType::Return),
            _ => self.make_token(TokenType::Ident)
        }
    }

    fn string(&mut self) -> Token {
        self.start += 1;

        while !self.peek_matches(&'"') && !self.is_eof() {
            // if self.peek_matches(&'\n') { // in case ewe need to track current line
            //     self.line += 1;
            // }
            self.advance();
        }

        if self.is_eof() {
            self.error(
                format!("Unterminated string. Token so far: {:?}",
                         self.make_token(TokenType::String)))
        }
        else {
            let res = self.make_token(TokenType::String);
            self.advance(); // consume the last "
            res
        }
    }

    fn comment(&mut self) -> Token {
        while !self.peek_matches(&'\n') && !self.is_eof() {
            self.advance();
        }

        if self.is_eof() {
            self.error(
                format!("Unterminated comment. Token so far: {:?}",
                         self.make_token(TokenType::Comment)))
        }
        else {
            let res = self.make_token(TokenType::Comment);
            // self.advance();
            res
        }
    }

    fn number(&mut self) -> Token {
        while self.peek_is_digit() {
            self.advance();
        }

        if self.peek_matches(&'.') {
            self.advance();
            while self.peek_is_digit() {
                self.advance();
            }
        }
        self.make_token(TokenType::Numeric)
    }

    fn ident(&mut self) -> Token {
        while self.peek_is_alpha() || self.peek_is_digit() {
            self.advance();
        }
        self.make_ident()
    }

    fn whitespace(&mut self) -> Token {
        // while self.peek_is_whitespace() && !self.peek_matches(&'\n') {
        //     self.advance();
        // }
        // let value = &self.src[self.start..self.current];
        // match value {
        //     "\n" => {
        //         // self.line += 1;
        //         self.make_token(TokenType::Whitespace)
        //     }
        //     _ => self.make_token(TokenType::Whitespace)
        // }
        self.make_token(TokenType::Whitespace)
    }

    fn eof(&self) -> Token {
        Token::new(
            TokenType::Eof,
            "".to_owned()
        )
    }

    fn error(&self, res: String) -> Token {
        Token::new(
            TokenType::Error,
            res
        )
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next_peek(&mut self) -> Option<&char> {
        self.chars.peek_nth(1)
    }

    fn peek_matches(&mut self, expected: &char) -> bool {
        match self.peek() {
            Some(c) => c == expected,
            None => false,
        }
    }

    fn peek_next_matches(&mut self, expected: &char) -> bool {
        match self.next_peek() {
            Some(c) => c == expected,
            None => false,
        }
    }

    fn matches(&mut self, expected: &char) -> bool {
        match self.peek() {
            Some(c) => {
                if c == expected {
                    self.advance();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn peek_is_digit(&mut self) -> bool{
        match self.peek() {
            Some(c) => Lexer::is_digit(*c),
            None => false,
        }
    }

    fn peek_is_alpha(&mut self) -> bool{
        match self.peek() {
            Some(c) => Lexer::is_alpha(*c),
            None => false,
        }
    }

    fn peek_is_whitespace(&mut self) -> bool{
        match self.peek() {
            Some(c) => Lexer::is_whitespace(*c),
            None => false,
        }
    }

    fn is_digit(c: char) -> bool {
        INTEGER_DIGITS[c as usize]
    }

    fn is_alpha(c: char) -> bool {
        IDENT_CHARS[c as usize]
    }

    fn is_whitespace(c: char) -> bool {
        WHITESPACE[c as usize]
    }
}