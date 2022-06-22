use crate::risp::shared::Op;
use super::{SyntaxError, AstNode, Lexer, Token, TokenKind};

/// Parses tokens from a Lexer into abstract syntax trees (ASTs).
pub struct Parser<'a> {
    /// A Lexer object. Allows the parser to have more control over the lexer.
    lexer: &'a mut Lexer<'a>,
    /// The current token that the parser is on.
    current_token: Token,
    /// The source to generate ASTs from. This should be the same as the lexer source.
    src: &'a str,
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    pub fn new(lexer: &'a mut Lexer<'a>, src: &'a str) -> Result<Self, SyntaxError> {
        Ok(Self {
            current_token: lexer.next()?,
            src,
            lexer,
        })
    }

    /// Advances the parser to the next lexer token
    #[inline]
    fn advance(&mut self) -> Result<(), SyntaxError> {
        self.current_token = self.lexer.next()?;
        Ok(())
    }

    /// Checks if current token is of a specific type, and then advances the parser
    fn expect(&mut self, kind: TokenKind) -> Result<(), SyntaxError> {
        if self.current_token.kind != kind {
            let error_msg = format!("expected {kind:?}, found {:?}", self.current_token.kind);
            return Err(SyntaxError(error_msg));
        }
        self.advance()
    }

    /// Parses an atom.
    ///
    /// An atom is the smallest unit of a parsed program.
    /// It can be an integer, a float, a string, a name, or even an operator
    fn parse_atom(&mut self) -> Result<AstNode, SyntaxError> {
        // Since tokens store their content as spans
        let content = &self.src[self.current_token.span.range()];
        let kind = self.current_token.kind;

        self.advance()?;

        let node = match &kind {
            // Integers and floats may have leading signs, but luckily Rust's `parse()` handles them.
            // It also correctly parses cases like `1.` and `0.`
            TokenKind::Int => AstNode::Int(content.parse().unwrap()),
            TokenKind::Float => AstNode::Float(content.parse().unwrap()),

            TokenKind::String => AstNode::Str(content.into()),

            TokenKind::Operator => {
                let op_kind = match &content[..] {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "*" => Op::Star,
                    "/" => Op::Slash,
                    _ => unreachable!(),
                };

                AstNode::Operator(op_kind)
            }

            TokenKind::Name => AstNode::Name(content.into()),


            // No other token should ever occur in an atom
            // `(` case is already covered in parse_expr
            t => {
                let error_msg = format!("unexpected {t:?} while parsing atom");
                return Err(SyntaxError(error_msg))
            },
        };

        return Ok(node);
    }

    /// Parses a list
    ///
    /// A list contains zero or more expressions between a pair of parentheses.
    /// Note that this function returns a Vec<AstNode>, not an AstNode::List
    fn parse_list(&mut self) -> Result<Vec<AstNode>, SyntaxError> {
        self.expect(TokenKind::OpenParen)?;

        let mut elements: Vec<AstNode> = Vec::new();

        // Keep adding elements to the list while a close paren is not encountered,
        // The EOF check prevents infinite loops
        while self.current_token.kind != TokenKind::CloseParen
            && self.current_token.kind != TokenKind::EOF
        {
            elements.push(self.parse_expr()?);
        }

        // Expect a `)`, just in case it terminated at an EOF
        self.expect(TokenKind::CloseParen)?;

        return Ok(elements);
    }

    /// Parses an expression
    ///
    /// EXPR ::= LIST | ATOM
    pub fn parse_expr(&mut self) -> Result<AstNode, SyntaxError> {
        match self.current_token.kind {
            // Parses as a list if the item starts with a `(`.
            // This works because a list will always begin with a `(`.
            TokenKind::OpenParen => Ok(AstNode::Expr(self.parse_list()?)),

            TokenKind::EOF => Err(SyntaxError("unexpected EOF while parsing atom".into())),

            // Anything else is parsed as an atom
            _ => self.parse_atom(),
        }
    }

    /// Keeps parsing expressions until EOF is encountered.
    pub fn parse_exprs(&mut self) -> Result<Vec<AstNode>, SyntaxError> {
        let mut exprs = Vec::new();

        while !self.lexer.eof() {
            exprs.push(self.parse_expr()?);
        }

        Ok(exprs)
    }
}
