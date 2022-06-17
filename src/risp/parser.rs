use crate::risp::{Op, AstNode, EOFError, Error, ExpectError, ErrorKind, TokenKind, Lexer, Token};

/// Struct which represents a parser, that parses tokens into ASTs
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    src: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new parser
    pub fn new(mut lexer: Lexer<'a>, src: &'a str) -> Result<Self, ErrorKind> {
        Ok(Self {
            current_token: lexer.next()?,
            src,
            lexer,
        })
    }

    /// Advances the parser to the next lexer token
    #[inline]
    fn advance(&mut self) -> Result<(), ErrorKind> {
        self.current_token = self.lexer.next()?;
        Ok(())
    }

    /// Checks if current token is of a specific type, and then advances the parser
    fn expect(&mut self, kind: TokenKind) -> Result<(), ErrorKind> {
        if self.current_token.kind != kind {
            return Err(ExpectError(kind));
        }
        self.advance()
    }

    fn parse_sign(&self, s: &str) -> (String, bool) {
        match s.chars().nth(0) {
            Some('+') => (s[1..].into(), false),
            Some('-') => (s[1..].into(), true),
            _ => (s.into(), false)
        }
    }

    /// Parses an atom
    /// ATOM ::= EXPR | NUMBER | NAME | STRING
    fn parse_atom(&mut self) -> Result<AstNode, ErrorKind> {
        let content = self.src[self.current_token.span.range()].to_owned();
        let kind = self.current_token.kind;
        self.advance()?;

        let node = match &kind {
            TokenKind::Int => {
                let (content, neg) = self.parse_sign(&content);
                let mut num: i32 = content.parse().unwrap();
                
                if neg {
                    num = -num;
                }

                AstNode::Int(num)
            }

            TokenKind::Float => {
                let (content, neg) = self.parse_sign(&content);
                let mut num: f64 = content.parse().unwrap();
                
                if neg {
                    num = -num;
                }

                AstNode::Float(num)
            }

            TokenKind::String => AstNode::Str(content),

            TokenKind::Operator => {
                let op_kind = match &content[..] {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "*" => Op::Star,
                    "/" => Op::Slash,
                    _   => unreachable!()
                };

                AstNode::Operator(op_kind)
            }

            TokenKind::Name => AstNode::Name(content),

            TokenKind::EOF => return Err(EOFError("atom".to_owned())),

            t => return Err(Error(format!("Invalid token {t:?} in atom"))),
        };

        return Ok(node);
    }

    /// Parses a list
    /// LIST ::= '(' EXPR* ')'
    fn parse_list(&mut self) -> Result<Vec<AstNode>, ErrorKind> {
        self.expect(TokenKind::OpenParen)?;

        let mut elements: Vec<AstNode> = Vec::new();

        while self.current_token.kind != TokenKind::CloseParen && self.current_token.kind != TokenKind::EOF {
            elements.push(self.parse_expr()?);
        }

        self.expect(TokenKind::CloseParen)?;
        return Ok(elements);
    }

    /// Parses an expression
    /// EXPR ::= LIST | ATOM
    pub fn parse_expr(&mut self) -> Result<AstNode, ErrorKind> {
        match self.current_token.kind {
            TokenKind::OpenParen => Ok(AstNode::Expr(self.parse_list()?)),
            TokenKind::EOF => return Err(EOFError("expr".to_owned())),
            _ => self.parse_atom(),
        }
    }

    /// Parses expressions until EOF
    pub fn parse_exprs(&mut self) -> Result<Vec<AstNode>, ErrorKind> {
        let mut exprs = Vec::new();
        
        while !self.lexer.eof() {
            exprs.push(self.parse_expr()?);
        }

        Ok(exprs)
    }
}
