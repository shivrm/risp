use std::fmt;

mod interpreter;
mod lexer;
mod parser;
mod rispstd;
mod utils;

pub use self::interpreter::Intepreter;
pub use self::lexer::Lexer;
pub use self::parser::Parser;
pub use self::utils::Span;

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
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}

#[derive(Clone)]
pub enum AstNode {
    Number(i32),
    Name(String),
    String(String),
    Expr(Vec<AstNode>),
}

#[derive(Clone)]
pub enum Type {
    Number(i32),
    String(String),
    List(Vec<Type>),
    BuiltinFn(&'static dyn Fn(Vec<Type>) -> Vec<Type>),
    Null,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Number(n) => write!(f, "{n}"),
            Type::String(s) => write!(f, "{s}"),
            Type::BuiltinFn(_) => write!(f, "<Builtin Function>"),
            Type::List(elems) => {
                let mut iter = elems.iter();

                match iter.next() {
                    Some(el) => write!(f, "[{el}")?,
                    None => write!(f, "[")?,
                }

                for el in iter {
                    write!(f, ", {el}")?;
                }

                write!(f, "]")
            }
            Type::Null => write!(f, "Null"),
        }
    }
}

pub fn to_ast(text: &str) -> Result<AstNode, Error> {
    let lexer = Lexer::new(text);
    Parser::new(lexer, text)?.parse_expr()
}
