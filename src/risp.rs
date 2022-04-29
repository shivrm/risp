use std::any::Any;
pub mod lexer;

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