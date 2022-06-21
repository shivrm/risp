mod astgen;
mod runtime;
mod shared;

pub use astgen::*;
pub use runtime::{ErrorKind, Interpreter, RuntimeError, RispType, Type};
pub use shared::Op;

pub fn to_ast(text: &str) -> Result<Vec<AstNode>, SyntaxError> {
    let mut lexer = Lexer::new(text);
    let mut parser = Parser::new(&mut lexer, text)?;

    parser.parse_exprs()
}
