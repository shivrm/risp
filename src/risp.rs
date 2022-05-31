mod interpreter;
mod lexer;
mod parser;
mod utils;
mod types;

pub use self::interpreter::Intepreter;
pub use self::lexer::Lexer;
pub use self::parser::Parser;
pub use self::utils::Span;
pub use self::types::{ Type, RispType };

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Unexpected char {0} while lexing token")]
    LexError(char),

    #[error("Unexpected EOF while reading {0}")]
    EOFError(String),

    #[error("Expected {0:?}")]
    ExpectError(Kind),

    #[error("Unknown name {0}")]
    NameError(String),

    #[error("{0} is not callable")]
    CallError(String),

    #[error("{0}")]
    Error(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Name,
    Number,
    String,
    OpenParen,
    CloseParen,
    Operator,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}

#[derive(Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash
}

#[derive(Clone)]
pub enum AstNode {
    Number(i32),
    Name(String),
    String(String),
    Operator(Op),
    Expr(Vec<AstNode>),
}

pub fn to_ast(text: &str) -> Result<AstNode, Error> {
    let lexer = Lexer::new(text);
    Parser::new(lexer, text)?.parse_expr()
}
