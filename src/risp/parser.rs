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
                details: format!("The parser expected {:?}, but found {:?}", kind, self.current_token)
            })
        }
        self.advance()
    }

    fn parse_atom(&mut self) -> Result<AstNode, Error> {
        let node = match &self.current_token {

            Token::OpenParen => return self.parse_expr(),
            Token::Number(value) => AstNode::Number(*value),
            Token::Name(value) => AstNode::Name(value.clone()),
            
            Token::CloseParen => return Err(Error {
                title: "Atom can not start with ')'".to_owned(),
                details: "The parser detected an atom that started with a closing parenthesis".to_owned()
            }),
            
            Token::EOF => return Err(Error {
                title: "Unexpected EOF while reading atom".to_owned(),
                details: "The parser unexpectedly encountered EOF while reading an atom".to_owned()
            })
        };
        self.advance()?;
        return Ok(node);
    }

    fn parse_list(&mut self) -> Result<Vec<AstNode>, Error> {
        self.expect(Token::OpenParen)?;
        
        let mut elements: Vec<AstNode> = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::EOF {
            elements.push(self.parse_expr()?);
        }

        self.expect(Token::CloseParen)?;
        return Ok(elements);
    }

    pub fn parse_expr(&mut self) -> Result<AstNode, Error> {
        
        match self.current_token {
            Token::OpenParen => Ok(AstNode::Expr(self.parse_list()?)),

            Token::EOF => return Err(Error {
                title: "Unexpected EOF while reading expression".to_owned(),
                details: "The parser unexpectedly encountered EOF while reading an expression".to_owned()
            }),

            _ => self.parse_atom()
        }
    }
}