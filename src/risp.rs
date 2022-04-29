pub mod lexer;
pub mod parser;

pub use self::lexer::Lexer;
pub use self::parser::Parser;

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