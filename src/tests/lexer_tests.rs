use crate::compiler::lexer::{Lexer};
use crate::compiler::token::{Token, TokenType};

#[test]
fn single_tokens() {
    let mut lx = Lexer::new(r"1 + 2 * 3");
    let tok = lx.next_token();
    println!("{:?}", tok);
    assert_eq!(tok, Token::new(TokenType::Numeric, "1".to_owned()));
    let _ = lx.next_token();
    let tok = lx.next_token();
    println!("{:?}", tok);
    assert_eq!(tok, Token::new(TokenType::Add, "+".to_owned()));
    let _ = lx.next_token();
    let tok = lx.next_token();
    println!("{:?}", tok);
    assert_eq!(tok, Token::new(TokenType::Numeric, "2".to_owned()));
    let _ = lx.next_token();
    let tok = lx.next_token();
    println!("{:?}", tok);
    assert_eq!(tok, Token::new(TokenType::Mul, "*".to_owned()));
    let _ = lx.next_token();
    let tok = lx.next_token();
    println!("{:?}", tok);
    assert_eq!(tok, Token::new(TokenType::Numeric, "3".to_owned()));
}

#[test]
fn numbers() {
    let mut lx = Lexer::new(r"1 45 3.14 0.667");
    let tokens = lx.tokenize_no_whitespace();
    // println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Numeric, "1".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Numeric, "45".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Numeric, "3.14".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::Numeric, "0.667".to_owned()));
}

#[test]
fn range() {
    let mut lx = Lexer::new(r"1..n");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Numeric, "1".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Range, "..".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ident, "n".to_owned()));
}

#[test]
fn strings() {
    let mut lx = Lexer::new(r#""hello,""world!""#);
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::String, "hello,".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::String, "world!".to_owned()));
}

#[test]
fn identifiers() {
    let mut lx = Lexer::new(r"foo BAR BaZ");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Ident, "foo".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Ident, "BAR".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ident, "BaZ".to_owned()));
}

#[test]
fn operators() {
    let mut lx = Lexer::new(r"> >> >= >>=");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:#?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Gt, ">".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Rshift, ">>".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ge, ">=".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::CompRshift, ">>=".to_owned()));
}

#[test]
// currently, the angled bracket is also recognized as an operator,
// meaning that its use as a bracket is not recognized, we will need
// to implement a check on operator fail, if it is using angled brackets,
// and switch to use them as brackets
fn brackets() {
    let mut lx = Lexer::new(r"(){}[]");
    // let mut lx = Lexer::new(r"(){}[]<>");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Lparen, "(".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Rparen, ")".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Lcurly, "{".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::Rcurly, "}".to_owned()));
    assert_eq!(tokens[4], Token::new(TokenType::Lsquare, "[".to_owned()));
    assert_eq!(tokens[5], Token::new(TokenType::Rsquare, "]".to_owned()));
}

#[test]
fn keywords() {
    let mut lx = Lexer::new(r"if else while for return");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::If, "if".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Else, "else".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::While, "while".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::For, "for".to_owned()));
    assert_eq!(tokens[4], Token::new(TokenType::Return, "return".to_owned()));
}

#[test]
fn comments() {
    let mut lx = Lexer::new("? this is a comment \n this isnt");
    let tokens = lx.tokenize_no_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Comment, "? this is a comment ".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Ident, "this".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ident, "isnt".to_owned()));
}