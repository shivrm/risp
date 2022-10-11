/// A simple span of lines, or a start and end position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// The start location that the span references in the source
    pub start: usize,
    /// The end location that the span references in the source
    pub end: usize,
}

impl Span {
    /// Creates a new span
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Converts the span to a [`Range<usize>`](core::ops::Range)
    #[inline]
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

/// The different types of tokens that may be present in the source code
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    /// An identifier
    Name,
    /// An integer
    Int,
    /// A floating point number
    Float,
    /// A string, without the surrounding quotes
    String,
    /// An opening parenthesis
    OpenParen,
    /// A closing parenthesis
    CloseParen,
    /// A quote
    Quote,
    /// An operator
    Operator,
    /// Represents the end of the source string
    EOF,
}


/// A struct that represents a token, which is a small sequence of 
/// characters with a particular meaning.
/// 
/// A token stores its kind, as well as the position in which it
/// appears in the source string.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,
    /// The position of the token in the source string.
    pub span: Span,
}
