use crate::risp::{ Error, Kind, Token };
use crate::risp::utils::Span;
use std::str::Chars;

/// Struct that represents a lexer, used for producing tokens from a string of text
pub struct Lexer<'a> {
    /// Iterator over the characters of the text
    chars : Chars<'a>,
    /// Current position that the lexer references to generate tokens
    pos   : usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(text: &'a str) -> Self {
        Self {
            chars : text.chars(),
            pos   : 0,
        }
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
                    self.pos  += c.len_utf8();
                }

                _ => return Span::new(start, self.pos),
            }
        }
    }

    /// Advances the lexer to the next character
    #[inline]
    fn adv(&mut self) {
        self.pos +=
            match self.chars.next() {
                Some(c) => c,
                None => return,
            }
                .len_utf8();
    }

    /// Gets the next token from the text and returns it
    #[inline]
    pub fn next(&mut self) -> Result<Token, Error> {
        let start = self.pos;
        let next =
            self
                .chars
                .clone()
                .next();
        let next = match next {
            None => {
                return Ok(Token {
                    kind : Kind::EOF,
                    span : Span::new(self.pos, self.pos)
                })
            }
            Some(c) => c
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
                Ok(Token { span, kind: Kind::Number })
            }

            // Change this to your liking
            // Currently matches anything like: abdc_01, __99, _0_9_a, abc, _ etc.
            'a'..='z' | 'A'..='Z' | '_' => {
                let span =
                    self.take_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
                Ok(Token { span, kind: Kind::Name })
            }
            
            // Miscellaneous single-character tokens
            c => {
                self.adv();
                let span = Span::new(start, self.pos);
                match c {
                    '(' => Ok(Token { span, kind: Kind::OpenParen }),
                    ')' => Ok(Token { span, kind: Kind::CloseParen }),
                    _   => Err(Error::LexError(c)),
                }
            }
        }
    }
}
