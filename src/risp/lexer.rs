use std::str::Chars;
use crate::risp::{Token, TokenKind};

pub struct Lexer<'a> {
    chars: Chars<'a>
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Lexer {
            chars: text.chars()
        }
    }

    #[inline]
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
    
    #[inline]
    fn current_char(&self) -> Option<char> {
        self.chars.clone().next()
    }
    
    fn read_number(&mut self) -> i32 {
        let mut num_as_string = String::new();
        while let c @ Some('0'..='9') = self.current_char() {
            num_as_string.push(c.unwrap());
            self.advance();
        }

        num_as_string.parse().unwrap()
    }

    fn read_name(&mut self) -> String {
        let mut num_as_string = String::new();
        while let c @ Some( 'A'..='Z' | 'a'..='z' ) = self.current_char() {
            num_as_string.push(c.unwrap());
            self.advance();
        }

        num_as_string
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(c) = self.current_char() {
            
            // Skip character if it is whitespace
            if c.is_whitespace() {
                self.advance();
                self.next_token()


            } else if c.is_numeric() {
                let value = self.read_number();
                Token {
                    kind: TokenKind::Number,
                    value: Box::new(value)
                }


            } else if c.is_alphabetic() {
                let value = self.read_name();
                Token {
                    kind: TokenKind::Name,
                    value: Box::new(value)
                }

            // Miscellaneous single-character tokens
            } else {
                self.advance();
                let kind = match c {
                    '(' => TokenKind::OpenParen,
                    ')' => TokenKind::CloseParen,
                    _ => panic!("Unknown character {c}")
                };
                
                Token {
                    kind,
                    value: Box::new(c)
                }
            }
        } else {
            panic!("EOF")
        }
    }
}