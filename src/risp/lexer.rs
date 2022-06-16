use crate::risp::token::Span;
use crate::risp::{Error, Kind, Token};
use std::str::Chars;

/// Struct that represents a lexer, used for producing tokens from a string of text
pub struct Lexer<'a> {
    /// Iterator over the characters of the text
    chars: Chars<'a>,
    /// Current position that the lexer references to generate tokens
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(text: &'a str) -> Self {
        Self {
            chars: text.chars(),
            pos: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.chars.clone().next().is_none()
    }

    /// Takes characters from the lexer while a predicate is met
    #[inline]
    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> Span {
        let start = self.pos;

        loop {
            // Character iterator is cloned for better performance
            let mut clone = self.chars.clone();

            match clone.next() {
                Some(c) if predicate(c) => {
                    self.chars = clone;
                    self.pos += c.len_utf8();
                }

                _ => return Span::new(start, self.pos),
            }
        }
    }

    /// Advances the lexer to the next character
    #[inline]
    fn adv(&mut self) {
        self.pos += match self.chars.next() {
            Some(c) => c,
            None => return,
        }
        .len_utf8();
    }

    /// Gets the next token from the text and returns it
    #[inline]
    pub fn next(&mut self) -> Result<Token, Error> {
        let start = self.pos;
        let next = self.chars.clone().next();

        let next = match next {
            None => {
                return Ok(Token {
                    kind: Kind::EOF,
                    span: Span::new(self.pos, self.pos),
                })
            }
            Some(c) => c,
        };

        match next {
            // Skip character if it is whitespace
            c if c.is_whitespace() => {
                self.take_while(|c| c.is_whitespace());
                self.next()
            }

            // Todo: Use a proper number parser here
            '0'..='9' => {
                let span = self.take_while(|c| matches!(c, '0'..='9'));
                
                if let Some('.') = self.chars.clone().next() {
                    self.adv();
                    self.take_while(|c| matches!(c, '0'..='9'));

                    Ok(Token {
                        span: Span::new(start, self.pos),
                        kind: Kind::Float
                    })
                } else {
                    Ok(Token {
                        span,
                        kind: Kind::Number,
                    })
                }

            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let span =
                    self.take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
                Ok(Token {
                    span,
                    kind: Kind::Name,
                })
            }

            '+' | '-' => {
                self.adv();
                
                if let Some('0'..='9') = self.chars.clone().next() {
                    let kind = self.next()?.kind;
                    Ok(Token {
                        span: Span::new(start, self.pos),
                        kind
                    })
                } else {
                    Ok(Token {
                        span: Span::new(start, self.pos),
                        kind: Kind::Operator,
                    })
                }
            }

            '"' => {
                self.adv();
                // TODO: Add support for escape sequences (might need to split this into a seperate fn)
                let span = self.take_while(|c| !matches!(c, '"'));
                self.adv();
                Ok(Token {
                    span,
                    kind: Kind::String,
                })
            }

            // Miscellaneous single-character tokens
            c => {
                self.adv();
                let span = Span::new(start, self.pos);
                let kind = match c {
                    '(' => Kind::OpenParen,
                    ')' => Kind::CloseParen,
                    '*' | '/' => Kind::Operator,
                    _ => return Err(Error::LexError(c)),
                };

                Ok(Token { span, kind })
            }
        }
    }
}
