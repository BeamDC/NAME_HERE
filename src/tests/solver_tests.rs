use crate::compiler::lexer::Token;
use crate::compiler::solver::Solver;

#[test]
fn test_shunting_yard() {
    let mut solver = Solver::new("1 + 2 * 3");
    solver.shunting_yard();
    println!("{:?}", solver.output);
    println!("{:?}", solver.holding);
    assert_eq!(solver.output[0], Token::Numeric("1".to_owned()));
    assert_eq!(solver.output[1], Token::Numeric("2".to_owned()));
    assert_eq!(solver.output[2], Token::Numeric("3".to_owned()));
    assert_eq!(solver.output[3], Token::Operator("*".to_owned()));
    assert_eq!(solver.output[4], Token::Operator("+".to_owned()));
}

#[test]
fn test_parenthesis_front() {
    let mut solver = Solver::new("(1 + 2) * 3");
    solver.shunting_yard();
    assert_eq!(solver.output[0], Token::Numeric("1".to_owned()));
    assert_eq!(solver.output[1], Token::Numeric("2".to_owned()));
    assert_eq!(solver.output[2], Token::Operator("+".to_owned()));
    assert_eq!(solver.output[3], Token::Numeric("3".to_owned()));
    assert_eq!(solver.output[4], Token::Operator("*".to_owned()));
}

#[test]
fn test_parenthesis_back() {
    let mut solver = Solver::new("1 + (2 * 3)");
    solver.shunting_yard();
    assert_eq!(solver.output[0], Token::Numeric("1".to_owned()));
    assert_eq!(solver.output[1], Token::Numeric("2".to_owned()));
    assert_eq!(solver.output[2], Token::Numeric("3".to_owned()));
    assert_eq!(solver.output[3], Token::Operator("*".to_owned()));
    assert_eq!(solver.output[4], Token::Operator("+".to_owned()));
}

#[test]
fn test_complex() {
    let mut solver = Solver::new("3.14 >= (2 << 1) * -4");
    solver.shunting_yard();
    assert_eq!(solver.output[0], Token::Numeric("3.14".to_owned()));
    assert_eq!(solver.output[1], Token::Numeric("2".to_owned()));
    assert_eq!(solver.output[2], Token::Numeric("1".to_owned()));
    assert_eq!(solver.output[3], Token::Operator("<<".to_owned()));
    assert_eq!(solver.output[4], Token::Numeric("4".to_owned()));
    assert_eq!(solver.output[5], Token::Operator("u-".to_owned()));
    assert_eq!(solver.output[6], Token::Operator("*".to_owned()));
    assert_eq!(solver.output[7], Token::Operator(">=".to_owned()));
}