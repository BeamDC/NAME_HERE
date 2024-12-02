/*
 * read in the buffer contents. split them into tokens
 *
 * Shoutout Robert Nystrom / Diego Freijo for the inspiration
 * repo -
 * book -
 */
use crate::vm::token::{Token, TokenType};
use itertools::{peek_nth, PeekNth};
use std::str::Chars;
use crate::constants::{INTEGER_DIGITS, IDENT_CHARS, WHITESPACE};

pub struct Lexer<'a> {
    pub src: &'a str,
    chars: PeekNth<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
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

// todo: allow the lexer to have its source changed, make parse return the tokens instead of storing them
impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src,
            chars     : peek_nth(src.chars()),
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
                _ if is_digit(c) => self.number(),
                _ if is_alpha(c) => self.ident(),
                _ if is_whitespace(c) => self.whitespace(),

                // match single chars
                '(' => self.make_token(TokenType::Lparen),
                ')' => self.make_token(TokenType::Rparen),
                '{' => self.make_token(TokenType::Lcurly),
                '}' => self.make_token(TokenType::Rcurly),
                '[' => self.make_token(TokenType::Lsquare),
                ']' => self.make_token(TokenType::Rsquare),
                ';' => self.make_token(TokenType::Semicolon),
                ':' => self.make_token(TokenType::Colon),
                ',' => self.make_token(TokenType::Comma),


                // match multi chars
                '.' |
                '+' |
                '-' |
                '*' |
                '/' |
                '%' |
                '<' |
                '>' |
                '&' |
                '|' |
                '^' |
                '~' |
                '!' |
                '=' => self.operator(c),

                // string
                '"' => self.string(),

                // char
                '\'' => self.char(),

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

    fn match_token(&mut self, expected: &char, on_match: TokenType, otherwise: TokenType) -> Token {
        if self.matches(expected) {
            self.make_token(on_match)
        }else {
            self.make_token(otherwise)
        }
    }

    fn match_multiple_token( &mut self, expected: Vec<&str>, on_match: Vec<TokenType>, otherwise: TokenType) -> Token {
        for (i, c) in expected.iter().enumerate() {
            if self.long_matches(c) {
                return self.make_token(on_match[i]);
            }
        }
        self.make_token(otherwise)
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
    fn char(&mut self) -> Token {
        self.start += 1;

        while !self.peek_matches(&'\'') && !self.is_eof() {
            // if self.peek_matches(&'\n') { // in case ewe need to track current line
            //     self.line += 1;
            // }
            self.advance();
        }

        if self.is_eof() {
            self.error(
                format!("Unterminated char. Token so far: {:?}",
                         self.make_token(TokenType::Char)))
        }
        else {
            let res = self.make_token(TokenType::Char);
            self.advance(); // consume the last '
            res
        }
    }

    fn comment(&mut self) -> Token {
        while !self.peek_matches(&'\n') && !self.peek_matches(&'\0') && !self.is_eof() {
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

        if self.peek_matches(&'.') && self.peek_next_matches(&'.') {
            return self.make_token(TokenType::Numeric);
        }else if self.peek_matches(&'.') {
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

    fn operator(&mut self, c: char) -> Token {
        match c {
            '.' => self.match_multiple_token(
                vec![&"."],
                vec![TokenType::Range],
                TokenType::Dot
            ),

            '+' => self.match_multiple_token(
                vec![&"+",&"="],
                vec![TokenType::Inc, TokenType::CompAdd],
                TokenType::Add
            ),

            '-' => self.match_multiple_token(
                vec![&"-",&"=",&">"],
                vec![TokenType::Dec, TokenType::CompSub, TokenType::Arrow],
                TokenType::Sub
            ),

            '*' => self.match_multiple_token(
                vec![&"*",&"="],
                vec![TokenType::Exp, TokenType::CompMul],
                TokenType::Mul
            ),

            '/' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::CompDiv],
                TokenType::Div
            ),

            '%' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::CompMod],
                TokenType::Mod
            ),

            '<' => self.match_multiple_token(
                vec![&"<=",&"=",&"<"],
                vec![TokenType::CompLshift, TokenType::Leq, TokenType::Lshift],
                TokenType::Less
            ),

            '>' => self.match_multiple_token(
                vec![&">=",&"=",&">",],
                vec![TokenType::CompRshift, TokenType::Geq, TokenType::Rshift],
                TokenType::Greater
            ),

            '&' => self.match_multiple_token(
                vec![&"&",&"="],
                vec![TokenType::And, TokenType::CompBitAnd],
                TokenType::BitAnd
            ),

            '|' => self.match_multiple_token(
                vec![&"|",&"="],
                vec![TokenType::Or, TokenType::CompBitOr],
                TokenType::BitOr
            ),

            '^' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::CompBitXor],
                TokenType::BitXor
            ),

            '~' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::CompBitNot],
                TokenType::BitNot
            ),

            '!' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::Neq],
                TokenType::Bang
            ),

            '=' => self.match_multiple_token(
                vec![&"="],
                vec![TokenType::Equal],
                TokenType::Assign
            ),
            _ => { self.eof() }
        }
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

    fn peek_next(&mut self) -> Option<&char> {
        self.chars.peek_nth(1)
    }

    fn peek_matches(&mut self, expected: &char) -> bool {
        match self.peek() {
            Some(c) => c == expected,
            None => false,
        }
    }

    fn peek_next_matches(&mut self, expected: &char) -> bool {
        match self.peek_next() {
            Some(c) => c == expected,
            None => false,
        }
    }

    fn matches(&mut self, expected: &char) -> bool {
        match self.peek() {
            Some(c) => {
                if c != expected {
                    return false;
                }
                self.advance();
            }
            None => return false,
        }
        true
    }

    fn long_matches(&mut self, expected: &str) -> bool {
        for (i, c) in expected.chars().enumerate() {
            match self.chars.peek_nth(i) {
                Some(&peeked) if peeked == c => continue,
                _ => return false,
            }
        }
        for _ in 0..expected.len() {
            self.advance();
        }
        true
    }

    fn peek_is_digit(&mut self) -> bool{
        match self.peek() {
            Some(c) => is_digit(*c),
            None => false,
        }
    }

    fn peek_is_alpha(&mut self) -> bool{
        match self.peek() {
            Some(c) => is_alpha(*c),
            None => false,
        }
    }

    fn peek_is_whitespace(&mut self) -> bool{
        match self.peek() {
            Some(c) => is_whitespace(*c),
            None => false,
        }
    }
}