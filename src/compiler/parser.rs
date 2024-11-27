use std::vec::IntoIter;
// Builds an AST from a list of tokens
use crate::compiler::lexer::{Token, TokenType};

struct AstNode {
    token: Token,
    children: Vec<AstNode>,
}

impl AstNode {
    fn new(token: Token) -> AstNode {
        AstNode {
            token,
            children: vec![],
        }
    }
}

struct Parser {
    tokens: IntoIter<Token>,
    current: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter(),
            current: Token::new(TokenType::Unknown, "".to_owned()),
        }
    }

    fn crash_out() {
        panic!("[PARSER] Syntax Error");
    }

    fn consume(&mut self, token_type: TokenType){
        if self.current.token_type != token_type {
            Parser::crash_out();
        }
        self.current = self.tokens.next().unwrap_or(Token::new(TokenType::Unknown, "".to_owned()));
    }
}