//! This module parses RISP source code into abstract syntax trees
//! (or ASTs). The [`Lexer`] is used to split the source code into
//! [`Token`]s and the [`Parser`] processes these tokens into an
//! [`AstNode`]. 
//! 
//! Invalid syntax may cause a [`SyntaxError`].


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
    Symbol(String),

    Str(String),
    Operator(super::shared::Op),

    Expr(Vec<AstNode>),
    List(Vec<AstNode>),
}

pub struct SyntaxError(pub String);

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {}", self.0)
    }
}
