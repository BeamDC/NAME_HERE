use crate::constants::KEYWORDS;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    // Single char
    Comma,
    Colon,
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

    // Uncategorized Ops
    Dot, Range,

    // Mathematical
    Add, Inc, CompAdd,
    Sub, Dec, CompSub,
    Mul, Exp, CompMul,
    Div, CompDiv,
    Mod, CompMod,

    // Bitwise
    Lshift, CompLshift,
    Rshift, CompRshift,
    BitAnd, CompBitAnd,
    BitOr, CompBitOr,
    BitXor, CompBitXor,
    BitNot, CompBitNot,

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

    // Keywords
    Fn,
    Bool, True, False,
    Let, Const,
    If, Else,
    For, While, Loop,
    Nil,
    Return,

    // Literals
    Ident,
    String,
    Char,
    Numeric,

    // Misc
    Arrow,
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