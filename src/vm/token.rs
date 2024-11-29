const KEYWORDS: [&str; 13] = [
    "fn",
    "bool", "true", "false",
    "let", "const",
    "if", "else",
    "for", "while", "loop",
    "nil",
    "return"
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    // single char
    Comma,
    Dot,
    Semicolon,
    Lparen,
    Rparen,
    Lsquare,
    Rsquare,
    Lcurly,
    Rcurly,
    Langled,
    Rangled,
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logical
    Bang,
    Neq,
    Equal,
    Greater,
    Less,
    Geq,
    Leq,
    And,
    Or,

    // Code Symbols
    Function,
    Comment,

    // keywords
    Fn,
    Bool, True, False,
    Let, Const,
    If, Else,
    For, While, Loop,
    Nil,
    Return,

    // literals
    Ident,
    String,
    Numeric,

    // misc
    Error, Eof, Whitespace,
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

    pub fn is_keyword(&self) -> bool{
        KEYWORDS.contains(&self.value.as_str())
    }
}