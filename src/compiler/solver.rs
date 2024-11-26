/*
 * An expression solver using the Shunting Yard Algorithm
 * here, an expression is generalized as combination of values and operators
 */
use std::collections::{HashMap, VecDeque};
use crate::compiler::lexer::{Lexer, Operator, OperatorMap, Token};

pub struct Solver {
    operator_map: HashMap<String, Operator>,
    pub tokens: Vec<Token>,
    pub holding: VecDeque<Token>,
    pub output: VecDeque<Token>,
}

impl Solver {
    pub fn new(expr: &str) -> Solver {
        let mut lx = Lexer::new(expr);
        lx.parse();
        let tokens = lx.tokens_filter_whitespace();
        // println!("{:?}", tokens);
        Solver {
            operator_map: OperatorMap::new().operators,
            tokens,
            holding: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn shunting_yard(&mut self) {
        let mut prev_token: Token = Token::Numeric("0".to_owned());
        let mut pass: i32 = 0;
        for token in self.tokens.iter() {
            match token {
                Token::Numeric(_) => {
                    self.output.push_back(token.clone());
                    prev_token = self.output.back().unwrap().clone();
                },
                Token::Lparen(_) => {
                    self.holding.push_front(token.clone());
                    prev_token = self.holding.front().unwrap().clone();
                },
                Token::Rparen(_) => {
                    if self.holding.is_empty() {
                        panic!("[SOLVER] Unexpected Closed Paren");
                    }
                    // drain holding stack into output stack
                    while !self.holding.is_empty() &&
                        self.holding.front().unwrap().value() != "(" {
                        let op = self.holding.pop_front().unwrap();
                        match op {
                            Token::Lparen(_) => break,
                            _ => self.output.push_back(op),
                        }
                    }

                    if self.holding.is_empty() {
                        panic!("[SOLVER] No Open Paren");
                    }
                    //remove Lparen after drain
                    if let Some(Token::Lparen(_)) = self.holding.front() {
                        self.holding.pop_front();
                    }
                    prev_token = Token::Rparen(")".to_owned());
                },
                Token::Operator(op) => {
                    while !self.holding.is_empty() &&
                        self.holding.front().unwrap().value() != "(" {

                        match self.holding.front().unwrap() {
                            Token::Operator(hold_op) => {
                                let hold_op: &Operator = &self.operator_map[hold_op];

                                if hold_op.precedence >= self.operator_map[op].precedence {
                                    self.output.push_back(self.holding.pop_front().unwrap());
                                } else { break; }
                            },
                            _ => continue,
                        }
                    }
                    self.holding.push_front(token.clone());
                    prev_token = self.holding.front().unwrap().clone();
                    println!("{:?}", self.holding)
                },
                _ => panic!("Unexpected Token in Expression"),
            }
            pass += 1;
        }

        // drain holding stack one last time
        while !self.holding.is_empty() {
            self.output.push_back(self.holding.pop_front().unwrap());
        }
    }

    // pub fn solve(&mut self) -> f64 {
    //     // self.shunting_yard();
    //     // self.evaluate()
    // }
}