use crate::risp::{Op, AstNode, Error, Kind, Lexer, Token};

/// Struct which represents a parser, that parses tokens into ASTs
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    src: &'a str,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    pub fn new(mut lexer: Lexer<'a>, src: &'a str) -> Result<Self, Error> {
        Ok(Self {
            current_token: lexer.next()?,
            src,
            lexer,
        })
    }

    /// Advances the parser to the next lexer token
    #[inline]
    fn advance(&mut self) -> Result<(), Error> {
        self.current_token = self.lexer.next()?;
        Ok(())
    }

    /// Checks if current token is of a specific type, and then advances the parser
    fn expect(&mut self, kind: Kind) -> Result<(), Error> {
        if self.current_token.kind != kind {
            return Err(Error::ExpectError(kind));
        }
        self.advance()
    }

    /// Parses an atom
    /// ATOM ::= EXPR | NUMBER | NAME | STRING
    fn parse_atom(&mut self) -> Result<AstNode, Error> {
        let content = self.src[self.current_token.span.range()].to_owned();

        let node = match &self.current_token.kind {
            Kind::Number => AstNode::Number(content.parse().unwrap()),

            Kind::String => AstNode::String(content),

            Kind::Operator => {
                let op_kind = match &content[..] {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "*" => Op::Star,
                    "/" => Op::Slash
                };
                AstNode::Operator(op_kind)
            }

            Kind::Name => AstNode::Name(content),

            Kind::EOF => return Err(Error::EOFError("atom".to_owned())),

            t => return Err(Error::Error(format!("Invalid token {t:?} in atom"))),
        };

        self.advance()?;
        return Ok(node);
    }

    /// Parses a list
    /// LIST ::= '(' EXPR* ')'
    fn parse_list(&mut self) -> Result<Vec<AstNode>, Error> {
        self.expect(Kind::OpenParen)?;

        let mut elements: Vec<AstNode> = Vec::new();

        while self.current_token.kind != Kind::CloseParen && self.current_token.kind != Kind::EOF {
            elements.push(self.parse_expr()?);
        }

        self.expect(Kind::CloseParen)?;
        return Ok(elements);
    }

    /// Parses an expression
    /// EXPR ::= LIST | ATOM
    pub fn parse_expr(&mut self) -> Result<AstNode, Error> {
        match self.current_token.kind {
            Kind::OpenParen => Ok(AstNode::Expr(self.parse_list()?)),
            Kind::EOF => return Err(Error::EOFError("expr".to_owned())),
            _ => self.parse_atom(),
        }
    }
}
