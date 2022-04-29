use crate::risp::{Token, AstNode, TokenKind, Lexer};

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        Parser {
            current_token: lexer.next_token(),
            lexer
        }
    }
}