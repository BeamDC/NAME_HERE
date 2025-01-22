// compile a tokenized file into chunks of bytecode

use crate::compiler::lexer::Lexer;

pub struct Compiler<'a> {
    pub error: bool,
    lexer: Lexer<'a>,
    scope: i8,
}