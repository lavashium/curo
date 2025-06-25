use crate::token::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Token {
            kind,
            lexeme,
            span
        }
    }

    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}