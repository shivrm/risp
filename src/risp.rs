mod lexer;
mod parser;
mod interpreter;

pub use self::lexer::Lexer;
pub use self::parser::Parser;
pub use self::interpreter::Intepreter;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(i32),
    Name(String),
    OpenParen,
    CloseParen,
    EOF
}

#[derive(Clone)]
pub enum AstNode {
    Number(i32),
    Name(String),
    Expr(Vec<AstNode>)
}

#[derive(Clone)]
pub enum Type {
    Number(i32),
    List(i32),
    BuiltinFn(&'static dyn Fn(Vec<Type>) -> Vec<Type>),
    Null
}