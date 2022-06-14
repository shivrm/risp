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
pub use self::token::{ Kind, Span, Token };

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Unexpected char {0} while lexing token")]
    LexError(char),

    #[error("Unexpected EOF while reading {0}")]
    EOFError(String),

    #[error("Expected {0:?}")]
    ExpectError(token::Kind),

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
