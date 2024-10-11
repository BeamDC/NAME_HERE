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

const KEYWORDS: [&str; 8] = [
    "return", "if", "else", "for", "while", "bool", "false", "true"

];

enum Token {
    Keyword(Vec<char>),
    Ident(Vec<char>),
    String(Vec<char>),
    Lparen(char),
    Rparen(char),
    Lsquare(char),
    Rsquare(char),
    Lcurly(char),
    Rcurly(char),
    Langled(char),
    Rangled(char),
    EOF(char),
}

pub struct Lexer<'a> {
    src: &'a str,
    chars: Peekable<CharIndices<'a>>,
    current: Option<(usize, char)>,
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            src: "",
            chars: src.char_indices().peekable(),
            current: None,
        }
    }

    pub fn read_next(&mut self) {
        self.current = self.chars.next();
    }

    pub fn match_next(&self) -> Token {
        let tok: Token;
        match self.current.unwrap_or((0usize, '\0')) { // (usize, char) is the char and its position
            (_,'(') => {tok = Token::Lparen('(')}
            (_,')') => {tok = Token::Rparen(')')}
            (_,'[') => {tok = Token::Lsquare('[')}
            (_,']') => {tok = Token::Rsquare(']')}
            (_,'<') => {tok = Token::Langled('<')}
            (_,'>') => {tok = Token::Langled('>')}
            _ => {tok = Token::EOF('\0')},
        }
        tok
    }
}

pub fn tokenize(contents: &Vec<u8>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    tokens
}