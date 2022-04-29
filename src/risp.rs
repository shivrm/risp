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

pub enum AstNode {
    Number(i32),
    Name(String),
    Expr(Vec<AstNode>)
}

pub enum Type {
    Number(i32),
    BuiltinFn(&'static dyn Fn(Vec<Type>) -> Vec<Type>)
}