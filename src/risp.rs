mod ast;
mod vm;
mod shared;
mod stdlib;

pub use ast::*;
pub use vm::{ErrorKind, Interpreter, RuntimeError, Value};
pub use shared::Op;

pub fn to_ast(text: &str) -> Result<Vec<AstNode>, SyntaxError> {
    let mut lexer = Lexer::new(text);
    let mut parser = Parser::new(&mut lexer, text)?;

    parser.parse_exprs()
}
