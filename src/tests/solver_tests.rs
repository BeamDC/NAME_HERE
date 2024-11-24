use crate::compiler::lexer::{Lexer, Token};

#[test]
fn mathematical_expr() {
    let mut lx = Lexer::new(r"-2 + -4");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Operator("u-".to_owned()));
    assert_eq!(tokens[1], Token::Numeric("2".to_owned()));
    assert_eq!(tokens[2], Token::Operator("+".to_owned()));
    assert_eq!(tokens[3], Token::Operator("u-".to_owned()));
    assert_eq!(tokens[4], Token::Numeric("4".to_owned()));
}