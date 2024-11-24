use crate::compiler::lexer::{Lexer, Token};

#[test]
fn numbers() {
    let mut lx = Lexer::new(r"1 45 3.14 0.667");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Numeric("1".to_owned()));
    assert_eq!(tokens[1], Token::Numeric("45".to_owned()));
    assert_eq!(tokens[2], Token::Numeric("3.14".to_owned()));
    assert_eq!(tokens[3], Token::Numeric("0.667".to_owned()));
}

#[test]
fn strings() {
    let mut lx = Lexer::new(r#""hello,""world!""#);
    lx.parse();
    println!("{:?}", lx.tokens);
    assert_eq!(lx.tokens[0], Token::String("hello,".to_owned()));
    assert_eq!(lx.tokens[1], Token::String("world!".to_owned()));
}

#[test]
fn identifiers() {
    let mut lx = Lexer::new(r"foo BAR BaZ");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Ident("foo".to_owned()));
    assert_eq!(tokens[1], Token::Ident("BAR".to_owned()));
    assert_eq!(tokens[2], Token::Ident("BaZ".to_owned()));
}

#[test]
fn operators() {
    let mut lx = Lexer::new(r"-1+1 & <<");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Operator("u-".to_owned()));
    assert_eq!(tokens[2], Token::Operator("+".to_owned()));
    assert_eq!(tokens[4], Token::Operator("&".to_owned()));
    assert_eq!(tokens[5], Token::Operator("<<".to_owned()));
}

#[test]
// currently, the angled bracket is also recognized as an operator,
// meaning that its use as a bracket is not recognized, we will need
// to implement a check on operator fail, if it is using angled brackets,
// and switch to use them as brackets
fn brackets() {
    let mut lx = Lexer::new(r"(){}[]");
    // let mut lx = Lexer::new(r"(){}[]<>");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Lparen('('));
    assert_eq!(tokens[1], Token::Rparen(')'));
    assert_eq!(tokens[2], Token::Lcurly('{'));
    assert_eq!(tokens[3], Token::Rcurly('}'));
    assert_eq!(tokens[4], Token::Lsquare('['));
    assert_eq!(tokens[5], Token::Rsquare(']'));
    // assert_eq!(lx.tokens[6], Token::Langled('<'));
    // assert_eq!(lx.tokens[7], Token::Rangled('>'));
}

#[test]
fn keywords() {
    let mut lx = Lexer::new(r"if else while for return");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::Keyword("if".to_owned()));
    assert_eq!(tokens[1], Token::Keyword("else".to_owned()));
    assert_eq!(tokens[2], Token::Keyword("while".to_owned()));
    assert_eq!(tokens[3], Token::Keyword("for".to_owned()));
    assert_eq!(tokens[4], Token::Keyword("return".to_owned()));
}

#[test]
fn misc() {
    let mut lx = Lexer::new(r";,");
    lx.parse();
    let tokens = lx.tokens_filter_whitespace();
    println!("{:?}", tokens);
    assert_eq!(tokens[0], Token::EndOfLine(';'));
    assert_eq!(tokens[1], Token::Separator(','));
}