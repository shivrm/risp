mod lexer;
mod parser;
mod token;

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