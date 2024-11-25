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
const OPERATORS: [bool; 256] = make_lut(r"!$%^&*+-=#@|`/\<>~");
const IDENT_CHARS: [bool; 256] = make_lut(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_1234567890"
);
const KEYWORDS: [&str; 21] = [
    "if", "else", "for", "while",
    "bool", "false", "true",
    "i8", "i16", "i32", "i64",
    "u8", "u16", "u32", "u64",
    "f32", "f64",
    "char", "str",
    "return", "fn"
];

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Unknown(String),
    Keyword(String),
    Ident(String),
    String(String),
    Operator(String),
    Numeric(String),
    Separator(String),
    EndOfLine(String),
    Comment(String), // todo : add support for comments
    Lparen(String),
    Rparen(String),
    Lsquare(String),
    Rsquare(String),
    Lcurly(String),
    Rcurly(String),
    Langled(String),
    Rangled(String),
    Whitespace(String),
}

impl Token {
    pub fn value(&self) -> String {
        match self {
            // todo : add case to remove 'u' from unary operators
            // todo : add case to add quotes to strings
            Token::Unknown(value) |
            Token::Keyword(value) |
            Token::Ident(value) |
            Token::Numeric(value) |
            Token::Separator(value) |
            Token::EndOfLine(value) |
            Token::Comment(value) |
            Token::Lparen(value) |
            Token::Rparen(value) |
            Token::Lsquare(value) |
            Token::Rsquare(value) |
            Token::Lcurly(value) |
            Token::Rcurly(value) |
            Token::Langled(value) |
            Token::Rangled(value) |
            Token::Whitespace(value) => value.to_string(),

            Token::String(value) => format!("\"{}\"", value),

            Token::Operator(value) => {
                if value.starts_with("u") {
                    value[1..].to_string()
                } else {
                    value.to_string()
                }
            }
        }
    }
}

pub struct Lexer<'a> {
    pub src: &'a str,
    chars: Chars<'a>,
    pub tokens: Vec<Token>,
    operators: HashMap<String, (usize, usize)>,
}

// todo: allow the lexer to have its source changed, make parse return the tokens instead of storing them
impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        let mut operators: HashMap<String, (usize, usize)> = HashMap::new();
        // binary operators
        operators.insert("=".to_string(), (0, 2));
        operators.insert("*".to_string(), (6, 2));
        operators.insert("/".to_string(), (6, 2));
        operators.insert("%".to_string(), (6, 2));
        operators.insert("+".to_string(), (5, 2));
        operators.insert("-".to_string(), (5, 2));
        // bitwise operators
        operators.insert("<<".to_string(), (4, 2));
        operators.insert(">>".to_string(), (4, 2));
        operators.insert("&".to_string(), (3, 2));
        operators.insert("^".to_string(), (2, 2));
        operators.insert("|".to_string(), (1, 2));
        // unary operators
        operators.insert("u+".to_string(), (100, 1));
        operators.insert("u-".to_string(), (100, 1));
        operators.insert("~".to_string(), (100, 1));
        // logical operators
        operators.insert("!".to_string(), (100, 2));
        operators.insert("&&".to_string(), (100, 2));
        operators.insert("||".to_string(), (100, 2));
        // comparison operators
        operators.insert("==".to_string(), (0, 2));
        operators.insert("!=".to_string(), (0, 2));
        operators.insert("<".to_string(), (0, 2));
        operators.insert(">".to_string(), (0, 2));
        operators.insert("<=".to_string(), (0, 2));
        operators.insert(">=".to_string(), (0, 2));


        Lexer {
            src: "",
            chars: src.chars(),
            tokens: vec![],
            operators,
        }
    }

    pub fn parse(&mut self) {
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

        let mut prev_state: State = State::New;
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
        while current_char != '\0' || current_state != State::New{
            match current_state {
                State::New => { // Determine the type of the token to be created
                    current_value.clear();
                    current_token = Token::Unknown(String::new());
                    decimal_found = false;

                    if WHITESPACE[current_char as usize] {
                        next_state = State::WhiteSpace;
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
                    current_token = Token::Whitespace(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    // prev_state = State::WhiteSpace;
                    next_state = State::Complete;
                },
                State::String => {
                    // we have to watch for the case where the string is not closed
                    if current_char == '"' || current_char == '\0' {
                        current_char = self.chars.next().unwrap_or( '\0' );
                        // current_value.push(current_char);
                        current_token = Token::String(current_value.clone());
                        prev_state = State::String;
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
                            current_token = Token::Numeric(current_value.clone());
                            prev_state = State::Numeric;
                            next_state = State::Complete;
                        }
                    }

                },
                State::Operator => {
                    if OPERATORS[current_char as usize] {
                        current_value.push(current_char);
                        if self.operators.contains_key(&current_value) {
                            current_char = self.chars.next().unwrap_or( '\0' );
                        }
                        else {
                            current_value.pop();
                            if self.operators.contains_key(&current_value) {
                                current_token = Token::Operator(current_value.clone());
                                prev_state = State::Operator;
                                next_state = State::Complete;
                            }
                            else {
                                current_value.push(current_char);
                                current_char = self.chars.next().unwrap_or( '\0' );
                            }
                        }
                    }else {
                        let valid_prev = prev_state != State::Numeric;

                        let valid_unary =
                            (current_value == "-" || current_value == "+") && valid_prev;

                        if self.operators.contains_key(&current_value) && valid_unary {
                            current_value.insert(0, 'u');
                            current_token = Token::Operator(current_value.clone());
                            prev_state = State::Operator;
                            next_state = State::Complete;
                        }
                        else if self.operators.contains_key(&current_value) {
                            current_token = Token::Operator(current_value.clone());
                            prev_state = State::Operator;
                            next_state = State::Complete;
                        }
                        else {
                            panic!("Unexpected operator {:?}", current_value);
                        }
                    }
                },
                State::Lparen => {
                    current_value.push(current_char);
                    current_token = Token::Lparen(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance += 1;
                    prev_state = State::Lparen;
                    next_state = State::Complete;
                },
                State::Rparen => {
                    current_value.push(current_char);
                    current_token = Token::Rparen(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    paren_balance -= 1;
                    prev_state = State::Rparen;
                    next_state = State::Complete;
                },
                State::Lsquare => {
                    current_value.push(current_char);
                    current_token = Token::Lsquare(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance += 1;
                    prev_state = State::Lsquare;
                    next_state = State::Complete;
                },
                State::Rsquare => {
                    current_value.push(current_char);
                    current_token = Token::Rsquare(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    square_balance -= 1;
                    prev_state = State::Rsquare;
                    next_state = State::Complete;
                },
                State::Lcurly => {
                    current_value.push(current_char);
                    current_token = Token::Lcurly(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance += 1;
                    prev_state = State::Lcurly;
                    next_state = State::Complete;
                },
                State::Rcurly => {
                    current_value.push(current_char);
                    current_token = Token::Rcurly(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    curly_balance -= 1;
                    prev_state = State::Rcurly;
                    next_state = State::Complete;
                },
                State::Langled => {
                    current_value.push(current_char);
                    current_token = Token::Langled(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance += 1;
                    prev_state = State::Langled;
                    next_state = State::Complete;
                },
                State::Rangled => {
                    current_value.push(current_char);
                    current_token = Token::Rangled(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    angled_balance -= 1;
                    prev_state = State::Rangled;
                    next_state = State::Complete;
                },
                State::Separator => {
                    current_value.push(current_char);
                    current_token = Token::Separator(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    prev_state = State::Separator;
                    next_state = State::Complete;
                },
                State::EndOfLine => {
                    current_value.push(current_char);
                    current_token = Token::EndOfLine(current_value.clone());
                    current_char = self.chars.next().unwrap_or( '\0' );
                    prev_state = State::EndOfLine;
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
                        prev_state = State::Name;
                        next_state = State::Complete;
                    }
                },
                State::Comment => {
                    if current_char == '\n' || current_char == '\0' {
                        current_token = Token::Comment(current_value.clone());
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
        self.tokens
            .iter()
            .filter(|&t| match t {
                Token::Whitespace(_) => false,
                _ => true,
            })
            .cloned()
            .collect()
    }
}