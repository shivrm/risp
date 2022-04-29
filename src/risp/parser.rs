use crate::risp::{Token, AstNode, TokenKind, Lexer};

pub struct Parser<'a> {
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

    fn parse_atom(&mut self) -> AstNode {
        let node = match self.current_token.kind {
            TokenKind::OpenParen => return self.parse_expr(),
            TokenKind::Number => AstNode::Number(*self.current_token.value.downcast_ref().unwrap()),
            TokenKind::Name => AstNode::Name(self.current_token.value.downcast_ref::<String>().unwrap().clone()),
            TokenKind::CloseParen => panic!("Atom can not start with closing paren"),
            TokenKind::EOF => panic!("Unexpected EOF while reading atom")
        };
        self.advance();
        return node;
    }

    fn parse_list(&mut self) -> Vec<AstNode> {
        self.expect(TokenKind::OpenParen);
        
        let mut elements: Vec<AstNode> = Vec::new();
        while self.current_token.kind != TokenKind::CloseParen && self.current_token.kind != TokenKind::EOF {
            elements.push(self.parse_expr());
        }

        self.expect(TokenKind::CloseParen);
        return elements;
    }

    pub fn parse_expr(&mut self) -> AstNode {
        
        match self.current_token.kind {
            TokenKind::OpenParen => AstNode::Expr(self.parse_list()),
            TokenKind::EOF => panic!("Unexpected EOF while parsing expression"),
            _ => self.parse_atom()
        }
    }
}