mod lexer;
mod parser;
mod token;

use std::fmt;

pub use lexer::Lexer;
pub use parser::Parser;
pub use token::{Token, TokenKind};

#[derive(Clone, Debug)]
pub enum AstNode {
    Int(i32),
    Float(f64),
    Name(String),
    Str(String),
    Operator(super::shared::Op),
    Expr(Vec<AstNode>),
}

pub struct SyntaxError(pub String);

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {}", self.0)
    }
}