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

    fn parse_list(&mut self) -> Vec<AstNode> {
        self.expect(TokenKind::OpenParen);
        
        let mut elements: Vec<AstNode> = Vec::new();
        while self.current_token.kind != TokenKind::CloseParen {
            elements.push(self.parse_expr());
        }

        self.expect(TokenKind::CloseParen);
        return elements;
    }

    fn parse_expr(&mut self) -> AstNode {
        todo!();
    }
}