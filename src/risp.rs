use std::any::Any;
pub mod lexer;
pub mod parser;

pub use self::lexer::Lexer;
pub use self::parser::Parser;

#[derive(PartialEq, Eq, Debug)]
pub enum TokenKind {
    Number,
    Name,
    OpenParen,
    CloseParen,
}

pub struct Token {
    kind: TokenKind,
    value: Box<dyn Any>
}

pub enum AstNode {
    Number(i32),
    Name(String),
    Expr(Vec<AstNode>)
}