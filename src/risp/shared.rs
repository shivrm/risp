use std::fmt;
use super::TokenKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
}

pub struct SyntaxError(pub String);

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error: {}", self.0)
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ErrorKind {
    #[error("Unknown name {0}")]
    NameError(String),

    #[error("{0} is not callable")]
    CallError(String),

    #[error("{0} does not implement {1} for {2}")]
    OpError(String, String, String),

    #[error("{0}")]
    Error(String),
}