use std::collections::HashMap;

pub enum Operation {
    Binary(fn(f64, f64) -> f64),
    BBitwise(fn(u64, u64) -> u64),
    UBitwise(fn(u64) -> u64),
    Unary(fn(f64) -> f64),
    BLogical(fn(bool, bool) -> bool),
    ULogical(fn(bool) -> bool),
    Comparison(fn(f64, f64) -> bool),
}

pub struct Operator {
    pub precedence: usize,
    pub args: usize,
    pub operation: Operation,
}

impl Operator {
    pub fn new(precedence: usize, args: usize, operation: Operation) -> Operator {
        Operator {
            precedence,
            args,
            operation,
        }
    }
}

pub struct OperatorMap {
    pub operators: HashMap<String, Operator>,
}

impl OperatorMap {
    pub fn new() -> OperatorMap {
        let mut operators: HashMap<String, Operator> = HashMap::new();
        // binary operators
        operators.insert("*".to_string(), Operator::new(7, 2, Operation::Binary(mul)));
        operators.insert("/".to_string(), Operator::new(7, 2, Operation::Binary(div)));
        operators.insert("%".to_string(), Operator::new(7, 2, Operation::Binary(rem)));
        operators.insert("+".to_string(), Operator::new(6, 2, Operation::Binary(add)));
        operators.insert("-".to_string(), Operator::new(6, 2, Operation::Binary(sub)));
        // bitwise operators
        operators.insert("<<".to_string(), Operator::new(5, 2, Operation::BBitwise(lshift)));
        operators.insert(">>".to_string(), Operator::new(5, 2, Operation::BBitwise(rshift)));
        operators.insert("&".to_string(), Operator::new(4, 2, Operation::BBitwise(band)));
        operators.insert("^".to_string(), Operator::new(3, 2, Operation::BBitwise(bxor)));
        operators.insert("|".to_string(), Operator::new(2, 2, Operation::BBitwise(bor)));
        operators.insert("~".to_string(), Operator::new(100, 1, Operation::UBitwise(bnot)));
        // unary operators
        operators.insert("u+".to_string(), Operator::new(100, 1, Operation::Unary(pos)));
        operators.insert("u-".to_string(), Operator::new(100, 1, Operation::Unary(neg)));
        // logical operators
        operators.insert("!".to_string(), Operator::new(100, 1, Operation::ULogical(not)));
        operators.insert("&&".to_string(), Operator::new(100, 2, Operation::BLogical(and)));
        operators.insert("||".to_string(), Operator::new(100, 2, Operation::BLogical(or)));
        // comparison operators
        operators.insert("==".to_string(), Operator::new(1, 2, Operation::Comparison(eq)));
        operators.insert("!=".to_string(), Operator::new(1, 2, Operation::Comparison(neq)));
        operators.insert("<".to_string(), Operator::new(1, 2, Operation::Comparison(lt)));
        operators.insert(">".to_string(), Operator::new(1, 2, Operation::Comparison(gt)));
        operators.insert("<=".to_string(), Operator::new(1, 2, Operation::Comparison(leq)));
        operators.insert(">=".to_string(), Operator::new(1, 2, Operation::Comparison(geq)));
        // other ## Do nothing until there is a use for these
        // operators.insert("->".to_string(), Operator::new(100, 2));
        // operators.insert(".".to_string(), Operator::new(100, 2));
        // operators.insert("..".to_string(), Operator::new(100, 2));
        // operators.insert("..=".to_string(), Operator::new(100, 2));
        // operators.insert("=".to_string(), Operator::new(0, 2));
        OperatorMap {
            operators,
        }
    }
}

// operator operations
// mathematical operators
fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn sub(a: f64, b: f64) -> f64 {
    a - b
}
fn mul(a: f64, b: f64) -> f64 {
    a * b
}
fn div(a: f64, b: f64) -> f64 {
    a / b
}
fn rem(a: f64, b: f64) -> f64 {
    a % b
}
// bitwise operators
fn lshift(a: u64, b: u64) -> u64 {
    a << b
}
fn rshift(a: u64, b: u64) -> u64 {
    a >> b
}
fn band(a: u64, b: u64) -> u64 {
    a & b
}
fn bxor(a: u64, b: u64) -> u64 {
    a ^ b
}
fn bor(a: u64, b: u64) -> u64 {
    a | b
}
// unary operators
fn pos(a: f64) -> f64 {
    if a < 0.0 { -a }
    else { a }
}
fn neg(a: f64) -> f64 {
    -a
}
fn bnot(a: u64) -> u64 {
    !a
}
// logical operators
fn not(a: bool) -> bool {
    !a
}
fn and(a: bool, b: bool) -> bool {
    a && b
}
fn or(a: bool, b: bool) -> bool {
    a || b
}
// comparison operators
fn eq(a: f64, b: f64) -> bool {
    a == b
}
fn neq(a: f64, b: f64) -> bool {
    a != b
}
fn lt(a: f64, b: f64) -> bool {
    a < b
}
fn gt(a: f64, b: f64) -> bool {
    a > b
}
fn leq(a: f64, b: f64) -> bool {
    a <= b
}
fn geq(a: f64, b: f64) -> bool {
    a >= b
}
// other
// not sure yet