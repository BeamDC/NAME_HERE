// use crate::compiler::lexer::{Lexer, Token, TokenType};
//
// #[test]
// fn test_parser() {
//     let mut lx = Lexer::new(r"1 + 2 * 3");
//     lx.tokenize();
//     let tokens = lx.tokens_filter_whitespace();
//     println!("{:?}", tokens);
//     assert_eq!(tokens[0], Token::new(TokenType::Numeric, "1".to_owned()));
// }