use std::collections::HashMap;

pub enum Operation {
    Bin(fn(f64, f64) -> f64),
    BinBit(fn(u64, u64) -> u64),
    UnBit(fn(u64) -> u64),
    Un(fn(f64) -> f64),
    BinLog(fn(bool, bool) -> bool),
    UnLog(fn(bool) -> bool),
    Cmp(fn(f64, f64) -> bool),
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
        operators.insert("*".to_string(), Operator::new(7, 2, Operation::Bin(mul)));
        operators.insert("/".to_string(), Operator::new(7, 2, Operation::Bin(div)));
        operators.insert("%".to_string(), Operator::new(7, 2, Operation::Bin(rem)));
        operators.insert("+".to_string(), Operator::new(6, 2, Operation::Bin(add)));
        operators.insert("-".to_string(), Operator::new(6, 2, Operation::Bin(sub)));
        // bitwise operators
        operators.insert("<<".to_string(), Operator::new(5, 2, Operation::BinBit(lshift)));
        operators.insert(">>".to_string(), Operator::new(5, 2, Operation::BinBit(rshift)));
        operators.insert("&".to_string(), Operator::new(4, 2, Operation::BinBit(band)));
        operators.insert("^".to_string(), Operator::new(3, 2, Operation::BinBit(bxor)));
        operators.insert("|".to_string(), Operator::new(2, 2, Operation::BinBit(bor)));
        operators.insert("~".to_string(), Operator::new(100, 1, Operation::UnBit(bnot)));
        // unary operators
        operators.insert("u+".to_string(), Operator::new(100, 1, Operation::Un(pos)));
        operators.insert("u-".to_string(), Operator::new(100, 1, Operation::Un(neg)));
        // logical operators
        operators.insert("!".to_string(), Operator::new(100, 1, Operation::UnLog(not)));
        operators.insert("&&".to_string(), Operator::new(100, 2, Operation::BinLog(and)));
        operators.insert("||".to_string(), Operator::new(100, 2, Operation::BinLog(or)));
        // comparison operators
        operators.insert("==".to_string(), Operator::new(1, 2, Operation::Cmp(eq)));
        operators.insert("!=".to_string(), Operator::new(1, 2, Operation::Cmp(neq)));
        operators.insert("<".to_string(), Operator::new(1, 2, Operation::Cmp(lt)));
        operators.insert(">".to_string(), Operator::new(1, 2, Operation::Cmp(gt)));
        operators.insert("<=".to_string(), Operator::new(1, 2, Operation::Cmp(leq)));
        operators.insert(">=".to_string(), Operator::new(1, 2, Operation::Cmp(geq)));
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