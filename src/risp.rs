mod interpreter;
mod lexer;
mod parser;
mod token;
mod types;
pub mod rispstd;

pub use self::interpreter::Intepreter;
pub use self::lexer::Lexer;
pub use self::parser::Parser;
pub use self::types::{ Type, RispType, Op };
pub use self::token::{ TokenKind, Span, Token };
pub use ErrorKind::*;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorKind {
    #[error("Unexpected char {0} while lexing token")]
    LexError(char),

    #[error("Unexpected EOF while reading {0}")]
    EOFError(String),

    #[error("Expected {0:?}")]
    ExpectError(TokenKind),

    #[error("Unknown name {0}")]
    NameError(String),

    #[error("{0} is not callable")]
    CallError(String),

    #[error("{0} does not implement {1} for {2}")]
    OpError(String, String, String),

    #[error("{0}")]
    Error(String),
}


#[derive(Clone)]
pub enum AstNode {
    Int(i32),
    Float(f64),
    Name(String),
    Str(String),
    Operator(Op),
    Expr(Vec<AstNode>),
}

pub fn to_ast(text: &str) -> Result<Vec<AstNode>, ErrorKind> {
    let mut lexer = Lexer::new(text);
    let mut parser = Parser::new(&mut lexer, text)?;

    parser.parse_exprs()
}
