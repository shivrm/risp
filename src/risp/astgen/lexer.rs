use super::token::Span;
use super::{SyntaxError, Token, TokenKind as Kind};
use std::str::Chars;

/// Splits a source string into tokens
pub struct Lexer<'a> {
    /// Iterator over the characters of the source
    chars: Chars<'a>,
    /// Current position that the lexer uses to construct spans
    pos: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a source string
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
            pos: 0,
        }
    }

    /// Returns the current lexer character, or '\0' if the lexer is at the end of the source
    #[inline]
    pub fn current_char(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    /// Returns true if the lexer is at the end of the source
    #[inline]
    pub fn eof(&self) -> bool {
        self.chars.clone().next().is_none()
    }

    /// Advances the lexer to the next character
    #[inline]
    fn adv(&mut self) {
        self.pos += match self.chars.next() {
            Some(c) => c.len_utf8(),
            None => return,
        }
    }

    /// Advances the lexer while a predicate is met.
    ///
    /// The predicate is a function which takes a `char` as an argument and returns a `bool`.
    /// The return value determines whether the lexer should be advanced.
    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> Span {
        let start = self.pos;

        loop {
            // Character iterator is cloned for better performance
            let mut clone = self.chars.clone();

            match clone.next() {
                // Advance the lexer if the predicate is met
                Some(c) if predicate(c) => {
                    self.chars = clone;
                    self.pos += c.len_utf8();
                }

                _ => return Span::new(start, self.pos),
            }
        }
    }

    /// Gets the next token from the lexer. Since the lexer might throw an error,
    /// this method returns a `Result`
    #[inline]
    pub fn next(&mut self) -> Result<Token, SyntaxError> {
        let start = self.pos;

        /// A macro that makes it easier to create a token
        macro_rules! tok {
            ($kind:expr) => {
                Token {
                    span: Span::new(start, self.pos),
                    kind: $kind,
                }
            };
            ($kind:expr, $span:expr) => {
                Token {
                    span: $span,
                    kind: $kind,
                }
            };
        }

        let token = match self.current_char() {
            '\0' if self.eof() => {
                tok!(Kind::EOF)
            }

            // Skip character if it is whitespace
            c if c.is_whitespace() => {
                self.take_while(|c| c.is_whitespace());
                self.next()?
            }

            '0'..='9' => {
                self.take_while(|c| matches!(c, '0'..='9'));

                // If the number is followed by a `.`, treat it as a float
                if let Some('.') = self.chars.clone().next() {
                    self.adv();
                    self.take_while(|c| matches!(c, '0'..='9'));

                    tok!(Kind::Float)
                } else {
                    tok!(Kind::Int)
                }
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                self.take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
                tok!(Kind::Name)
            }

            // + and - are matched here because they might be the sign of a number.
            '+' | '-' => {
                self.adv();

                // When next token is a number, add this +/- to it. The sign will be parsed by the parser.
                if let Some('0'..='9') = self.chars.clone().next() {
                    let kind = self.next()?.kind;
                    tok!(kind)
                } else {
                    tok!(Kind::Operator)
                }
            }

            '"' => {
                self.adv();

                // TODO: Add support for escape sequences (might need to split this into a seperate fn)
                let span = self.take_while(|c| !matches!(c, '"'));
                self.adv();
                tok!(Kind::String, span)
            }

            // Miscellaneous single-character tokens
            c => {
                self.adv();

                let kind = match c {
                    '(' => Kind::OpenParen,
                    ')' => Kind::CloseParen,
                    '*' | '/' => Kind::Operator,
                    _ => {
                        let error_msg = format!("did not expect character {c:?}");
                        return Err(SyntaxError(error_msg));
                    }
                };

                tok!(kind)
            }
        };

        Ok(token)
    }
}
