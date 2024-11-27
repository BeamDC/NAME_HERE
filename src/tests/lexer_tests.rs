use crate::compiler::lexer::{Lexer, Token, TokenType};

#[test]
fn numbers() {
    let mut lx = Lexer::new(r"1 45 3.14 0.667");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Numeric, "1".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Numeric, "45".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Numeric, "3.14".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::Numeric, "0.667".to_owned()));
}

#[test]
fn strings() {
    let mut lx = Lexer::new(r#""hello,""world!""#);
    lx.tokenize();
    println!("{:?}", lx.tokens);
    assert_eq!(lx.tokens[0], Token::new(TokenType::String, "hello,".to_owned()));
    assert_eq!(lx.tokens[1], Token::new(TokenType::String, "world!".to_owned()));
}

#[test]
fn identifiers() {
    let mut lx = Lexer::new(r"foo BAR BaZ");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Ident, "foo".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Ident, "BAR".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ident, "BaZ".to_owned()));
}

#[test]
fn operators() {
    let mut lx = Lexer::new(r"-1+1 & <<");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Operator, "u-".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Operator, "+".to_owned()));
    assert_eq!(tokens[4], Token::new(TokenType::Operator, "&".to_owned()));
    assert_eq!(tokens[5], Token::new(TokenType::Operator, "<<".to_owned()));
}

#[test]
// currently, the angled bracket is also recognized as an operator,
// meaning that its use as a bracket is not recognized, we will need
// to implement a check on operator fail, if it is using angled brackets,
// and switch to use them as brackets
fn brackets() {
    let mut lx = Lexer::new(r"(){}[]");
    // let mut lx = Lexer::new(r"(){}[]<>");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Lparen, "(".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Rparen, ")".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Lcurly, "{".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::Rcurly, "}".to_owned()));
    assert_eq!(tokens[4], Token::new(TokenType::Lsquare, "[".to_owned()));
    assert_eq!(tokens[5], Token::new(TokenType::Rsquare, "]".to_owned()));
    // assert_eq!(lx.tokens[6], Token::Langled('<'));
    // assert_eq!(lx.tokens[7], Token::Rangled('>'));
}

#[test]
fn keywords() {
    let mut lx = Lexer::new(r"if else while for return");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Keyword, "if".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Keyword, "else".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Keyword, "while".to_owned()));
    assert_eq!(tokens[3], Token::new(TokenType::Keyword, "for".to_owned()));
    assert_eq!(tokens[4], Token::new(TokenType::Keyword, "return".to_owned()));
}

#[test]
fn comments() {
    let mut lx = Lexer::new("? this is a comment \n this isnt");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::Comment, "? this is a comment ".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Ident, "this".to_owned()));
    assert_eq!(tokens[2], Token::new(TokenType::Ident, "isnt".to_owned()));
}

#[test]
fn misc() {
    let mut lx = Lexer::new(r";,");
    lx.tokenize();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::new(TokenType::EndOfLine, ";".to_owned()));
    assert_eq!(tokens[1], Token::new(TokenType::Separator, ",".to_owned()));
}