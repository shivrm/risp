use std::str::Chars;
use crate::risp::{Token, Error};

/// Struct that represents a lexer, used for producing tokens from a string of text
pub struct Lexer<'a> {
    chars: Chars<'a> // Iterator over the characters of the text
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars: text.chars()
        }
    }

    /// Advances the lexer to the next character
    #[inline]
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
    
    /// Returns the current character
    #[inline]
    fn current_char(&self) -> Option<char> {
        self.chars.clone().next()
    }
    
    /// Reads a number (consecutive digits) from the text
    fn read_number(&mut self) -> i32 {
        let mut num_as_string = String::new();

        while let c @ Some('0'..='9') = self.current_char() {
            num_as_string.push(c.unwrap());
            self.advance();
        }

        num_as_string.parse().unwrap()
    }

    /// Reads a name (consecutive alphabets) from the text
    fn read_name(&mut self) -> String {
        let mut num_as_string = String::new();

        while let c @ Some( 'A'..='Z' | 'a'..='z' ) = self.current_char() {
            num_as_string.push(c.unwrap());
            self.advance();
        }

        num_as_string
    }

    /// Gets the next token from the text and returns it
    pub fn next_token(&mut self) -> Result<Token, Error> {
        if let Some(c) = self.current_char() {
            
            // Skip character if it is whitespace
            if c.is_whitespace() {
                self.advance();
                self.next_token()


            } else if c.is_numeric() {
                let value = self.read_number();
                Ok(Token::Number(value))


            } else if c.is_alphabetic() {
                let value = self.read_name();
                Ok(Token::Name(value))

            // Miscellaneous single-character tokens
            } else {
                self.advance();
                match c {
                    '(' => Ok(Token::OpenParen),
                    ')' => Ok(Token::CloseParen),
                    _ => Err(Error::LexError(c))
                }
            }
        } else {
            // Character is None, so we are at the end of text
            Ok(Token::EOF)
        }
    }
}