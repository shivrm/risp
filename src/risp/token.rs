/// A simple span of lines, or a start and end position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// The start location that the span references in the source
    pub start: usize,
    /// The end location that the span references in the source
    pub end: usize,
}

impl Span {
    /// Create a new span
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Convert the span to a [`Range<usize>`](core::ops::Range)
    #[inline]
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Name,
    Number,
    Float,
    String,
    OpenParen,
    CloseParen,
    Operator,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}