use crate::token::*;
use accessors::accessors;
use constructors::constructors;

#[accessors]
#[constructors]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

#[accessors]
#[constructors]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    span: Span,
}
