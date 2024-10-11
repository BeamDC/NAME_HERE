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

enum TokenType {
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
}

pub struct Token<'a> {
    value: &'a str,
    species: TokenType, // 'type' didn't work, so I found a synonym
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

    pub fn read_next(&self) {
        // todo: get the next token and set it as the current.
    }

    pub fn match_next(&self){
        // todo: get the next token and match it to its type
    }
}

pub fn tokenize(contents: &Vec<u8>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    tokens
}