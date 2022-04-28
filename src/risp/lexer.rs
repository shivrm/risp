use std::str::Chars;
use crate::risp::{Token, TokenKind};

struct Lexer<'a> {
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
}