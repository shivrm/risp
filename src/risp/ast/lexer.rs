use super::token::Span;
use super::{SyntaxError, Token, TokenKind as Kind};
use std::str::Chars;

/// A struct that scans through a source string and splits it into
/// [`Token`]s. 
pub struct Lexer<'a> {
    /// An iterator over the characters of the source string.
    chars: Chars<'a>,
    /// The current of the lexer
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

    /// The character that the lexer is currently on. Returns \0
    /// if the lexer is at the end of the source string. 
    #[inline]
    pub fn current_char(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    /// Returns `true` if the lexer has reached the end of the source
    /// string.
    #[inline]
    pub fn eof(&self) -> bool {
        self.chars.clone().next().is_none()
    }

    /// Advances the lexer to the next character.
    #[inline]
    fn adv(&mut self) {
        self.pos += match self.chars.next() {
            Some(c) => c.len_utf8(),
            None => return,
        }
    }

    /// Continuously advances the lexer while the current character
    /// meets a given predicate.
    /// 
    /// The predicate is a function which accepts a `char` as an
    /// argument and returns a boool
    /// 
    /// For example, this predicate returns true if the character
    /// is an uppercase letter:
    /// ```rust
    /// |c| matches!(c, 'A'..='Z')
    /// ```
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

    /// Gets the next [`Token`] from the lexer. This function will
    /// return [`Err`] if the lexer does not know how to handle a
    /// character.
    #[inline]
    pub fn next(&mut self) -> Result<Token, SyntaxError> {
        let start = self.pos;

        /// Used for conveniently creating tokens.
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

            // Skip whitespace characters and get the next token
            c if c.is_whitespace() => {
                self.take_while(|c| c.is_whitespace());
                self.next()?
            }

            // Match integers and floats, which start with a digit.
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

            // Matches identifiers, which start with an alphabet or an underscore.
            // Succeding characters may be an alphabet, a number, or an underscore.
            'a'..='z' | 'A'..='Z' | '_' => {
                self.take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
                tok!(Kind::Name)
            }

            // + and - might denote the sign of a number, so they are parsed seperately
            // from the other operators.
            '+' | '-' => {
                self.adv();

                // If it is followed by a number, parse it and put the sign as
                // part of the number token. This will be correcly parsed by
                // Rust's `.parse()` methods.
                if let Some('0'..='9') = self.chars.clone().next() {
                    let kind = self.next()?.kind;
                    tok!(kind)
                } else {
                    tok!(Kind::Operator)
                }
            }

            // Matches string literals, which start with a double quote
            // NOTE: The string token does not include the quotes.
            // TODO: Add support for escape sequences.
            '"' => {
                self.adv();

                let span = self.take_while(|c| !matches!(c, '"'));
                self.adv(); // Advance over the closing quote
                tok!(Kind::String, span)
            }

            // Matches miscellaneous single-character tokens
            c => {
                // Since character is already stored in `c`, we can advance
                // to the next character now itself.
                self.adv();

                let kind = match c {
                    '(' => Kind::OpenParen,
                    ')' => Kind::CloseParen,
                    '*' | '/' | '>' | '<' | '=' => Kind::Operator,
                    '\'' => Kind::Quote,
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
