use crate::risp::{Error, Token};
use std::str::Chars;

/// Struct that represents a lexer, used for producing tokens from a string of text
pub struct Lexer<'a> {
    chars: Chars<'a>, // Iterator over the characters of the text
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars: text.chars(),
        }
    }

    /// Takes characters from the lexer while a predicate is met
    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
        let mut taken = String::new();

        loop {
            // Character iterator is cloned for better performance
            let mut clone = self.chars.clone();

            match clone.next() {
                Some(c) if predicate(c) => {
                    taken.push(c);
                    self.chars = clone;
                }

                _ => return taken,
            }
        }
    }

    /// Advances the lexer to the next character
    #[inline]
    fn adv(&mut self) {
        self.chars.next();
    }

    /// Returns the current character
    #[inline]
    fn current_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Gets the next token from the text and returns it
    pub fn next_token(&mut self) -> Result<Token, Error> {
        if let Some(c) = self.current_char() {
            // Skip character if it is whitespace
            if c.is_whitespace() {
                self.take_while(|c| c.is_whitespace());
                self.next_token()

            } else if c.is_numeric() {
                let value = self.take_while(|c| c.is_numeric());
                Ok(Token::Number(value.parse().unwrap()))
            
            } else if c.is_alphabetic() {
                let value = self.take_while(|c| c.is_alphabetic());
                Ok(Token::Name(value))

            // Miscellaneous single-character tokens
            } else {
                self.adv();
                match c {
                    '(' => Ok(Token::OpenParen),
                    ')' => Ok(Token::CloseParen),
                    _ => Err(Error::LexError(c)),
                }
            }
        } else {
            // Character is None, so we are at the end of text
            Ok(Token::EOF)
        }
    }
}
