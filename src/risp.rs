mod astgen;
mod runtime;
mod shared;

pub use astgen::*;
pub use runtime::*;
pub use shared::{
    *,
    ErrorKind::*
};

pub fn to_ast(text: &str) -> Result<Vec<AstNode>, ErrorKind> {
    let mut lexer = Lexer::new(text);
    let mut parser = Parser::new(&mut lexer, text)?;

    parser.parse_exprs()
}
