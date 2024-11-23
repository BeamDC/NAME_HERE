use crate::compiler::lexer::{Lexer, Token};

#[test]
fn numbers() {
    let mut lx = Lexer::new(r"123 456 789 0");
    lx.parse();
    println!("{:?}", lx.tokens);
    assert_eq!(lx.tokens[0], Token::Numeric("123".to_owned()));
    assert_eq!(lx.tokens[1], Token::Numeric("456".to_owned()));
    assert_eq!(lx.tokens[2], Token::Numeric("789".to_owned()));
    assert_eq!(lx.tokens[3], Token::Numeric("0".to_owned()));
}