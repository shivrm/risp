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

    #[inline]
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.current_token.kind != kind {
            panic!("Expected {:?}, got {:?}", kind, self.current_token.kind);
        }
        self.advance();
    }
}