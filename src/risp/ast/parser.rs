use super::{AstNode, Lexer, SyntaxError, Token, TokenKind};
use crate::risp::shared::Op;

/// A struct that parses [`Token`]s from a [`Lexer`] into an abstract
/// syntax tree.
pub struct Parser<'a> {
    /// A [`Lexer`] object which generates tokens from the source string.
    lexer: &'a mut Lexer<'a>,
    /// The current token that the parser is on.
    current_token: Token,
    /// The source string to generate the ASTs from. This MUST be the same
    /// as the lexer source string.
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

    /// Advances the parser to the next token
    #[inline]
    fn advance(&mut self) -> Result<(), SyntaxError> {
        self.current_token = self.lexer.next()?;
        Ok(())
    }

    /// Advances the parser if the current [`Token`] is of a specific kind.
    /// Returns a [`SyntaxError`] otherwise.
    fn expect(&mut self, kind: TokenKind) -> Result<(), SyntaxError> {
        if self.current_token.kind != kind {
            let error_msg = format!("expected {kind:?}, found {:?}", self.current_token.kind);
            return Err(SyntaxError(error_msg));
        }
        self.advance()
    }

    /// Parses an atom. An atom can be an int, a float, a string,
    /// an operator, or an identifier.
    fn parse_atom(&mut self) -> Result<AstNode, SyntaxError> {
        // Tokens store their content using a Span. This line slices
        // the source string to get the characters corresponding
        // to that token.
        let content = &self.src[self.current_token.span.range()];
        let kind = self.current_token.kind;

        self.advance()?;

        let node = match &kind {
            // Parses integers and floats. The `.parse()` method can
            // handle leading signs as well as edge cases like `1.`
            // and `.05`.
            TokenKind::Int => AstNode::Int(content.parse().unwrap()),
            TokenKind::Float => AstNode::Float(content.parse().unwrap()),

            // Parses a string. The string token does not include the
            // surrounding quotes, so it can be directly casted into
            // a String.
            TokenKind::String => AstNode::Str(content.into()),

            // Parses an operator.
            TokenKind::Operator => {
                let op_kind = match &content[..] {
                    "+" => Op::Plus,
                    "-" => Op::Minus,
                    "*" => Op::Star,
                    "/" => Op::Slash,
                    "=" => Op::Equal,
                    ">" => Op::Greater,
                    "<" => Op::Less,

                    // The lexer will not allow any other character to
                    // correspond to an Operator token.
                    _ => unreachable!(),
                };
            
                AstNode::Operator(op_kind)
            }

            // Parses an identifier.
            TokenKind::Name => AstNode::Name(content.into()),

            // Parses a quote
            TokenKind::Quote => match self.parse_expr()? {
                AstNode::Expr(e) => AstNode::List(e),
                AstNode::Name(e) => AstNode::Symbol(e),
                t => {
                    let error_msg = format!("{t:?} can not be quoted");
                    return Err(SyntaxError(error_msg))
                }
            },

            // No other tokens are valid atoms.
            t => {
                let error_msg = format!("unexpected {t:?} while parsing atom");
                return Err(SyntaxError(error_msg));
            }
        };

        return Ok(node);
    }

    /// Parses a list of expressions. A list can contain zero or more
    /// expressions and is surrounded by a pair of parentheses.
    fn parse_list(&mut self) -> Result<Vec<AstNode>, SyntaxError> {
        self.expect(TokenKind::OpenParen)?;

        let mut elements: Vec<AstNode> = Vec::new();

        // Appends elements to the list while a closing parenthesis
        // is not encountered. The EOF check prevents infinite loops.
        while self.current_token.kind != TokenKind::CloseParen
            && self.current_token.kind != TokenKind::EOF
        {
            elements.push(self.parse_expr()?);
        }

        // Verify that a closing parenthesis was encountered, and
        // not EOF.
        self.expect(TokenKind::CloseParen)?;

        return Ok(elements);
    }

    /// Parses an expression. An expression may be a list, or an atom
    pub fn parse_expr(&mut self) -> Result<AstNode, SyntaxError> {
        match self.current_token.kind {
            // If the expression begins with a opening parenthesis,
            // then it is a list.
            TokenKind::OpenParen => Ok(AstNode::Expr(self.parse_list()?)),

            TokenKind::EOF => Err(SyntaxError("unexpected EOF while parsing atom".into())),

            // Anything else is parsed as an atom.
            _ => self.parse_atom(),
        }
    }

    /// Parses expressions until EOF is encountered.
    pub fn parse_exprs(&mut self) -> Result<Vec<AstNode>, SyntaxError> {
        let mut exprs = Vec::new();

        while !self.lexer.eof() {
            exprs.push(self.parse_expr()?);
        }

        Ok(exprs)
    }
}
