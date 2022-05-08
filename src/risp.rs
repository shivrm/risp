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

impl Type {
    pub fn repr(&self) -> String {
        match self {
            Type::Number(n) => n.to_string(),
            Type::String(s) => format!("\"{s:?}\""),

            Type::List(elems) =>  {
                let mut iter = elems.iter();
                let mut repr = String::from("[");

                match iter.next() {
                    Some(el) => repr += &el.repr(),
                    None => (())
                }

                for el in iter {
                    repr += ", ";
                    repr += &el.repr();
                }

                repr.push(']');
                repr
            }

            Type::BuiltinFn(_) => "<Builtin Function>".to_owned(),
            Type::Null => "".into()
        }
    }

    pub fn display(&self) -> String {
        match self {
            Type::String(s) => s.clone(),
            Type::Null => "Null".into(),
            _ => self.repr()
        }
    }
}

pub fn to_ast(text: &str) -> Result<AstNode, Error> {
    let lexer = Lexer::new(text);
    Parser::new(lexer, text)?.parse_expr()
}
