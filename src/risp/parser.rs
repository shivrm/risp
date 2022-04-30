use crate::risp::{Token, AstNode, Lexer, Error};

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, Error> {
        Ok(Parser {
            current_token: lexer.next_token()?,
            lexer
        })
    }

    #[inline]
    fn advance(&mut self) -> Result<(), Error> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, kind: Token) -> Result<(), Error> {
        if !variant_eq(&self.current_token, &kind) {
            return Err(Error {
                title: "Found wrong token".to_owned(),
                details: format!("The parser expected {}, but found {}", kind, self.current_token)
            })
        }
        self.advance()
    }

    fn parse_atom(&mut self) -> AstNode {
        let node = match &self.current_token {
            Token::OpenParen => return self.parse_expr(),
            Token::Number(value) => AstNode::Number(*value),
            Token::Name(value) => AstNode::Name(value.clone()),
            Token::CloseParen => panic!("Atom can not start with closing paren"),
            Token::EOF => panic!("Unexpected EOF while reading atom")
        };
        self.advance();
        return node;
    }

    fn parse_list(&mut self) -> Vec<AstNode> {
        self.expect(Token::OpenParen);
        
        let mut elements: Vec<AstNode> = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::EOF {
            elements.push(self.parse_expr());
        }

        self.expect(Token::CloseParen);
        return elements;
    }

    pub fn parse_expr(&mut self) -> AstNode {
        
        match self.current_token {
            Token::OpenParen => AstNode::Expr(self.parse_list()),
            Token::EOF => panic!("Unexpected EOF while parsing expression"),
            _ => self.parse_atom()
        }
    }
}