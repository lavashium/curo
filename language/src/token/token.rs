use crate::token::*;
use accessors::accessors;
use constructors::constructors;

#[macro_export]
macro_rules! combine_spans {
    ($first:expr, $second:expr) => {{
        let first = $first;
        let second = $second;

        let start_line = std::cmp::min(first.get_start_line(), second.get_start_line());

        let start_col = if first.get_start_line() == second.get_start_line() {
            std::cmp::min(first.get_start_col(), second.get_start_col())
        } else if first.get_start_line() < second.get_start_line() {
            first.get_start_col()
        } else {
            second.get_start_col()
        };

        let end_line = std::cmp::max(first.get_end_line(), second.get_end_line());

        let end_col = if first.get_end_line() == second.get_end_line() {
            std::cmp::max(first.get_end_col(), second.get_end_col())
        } else if first.get_end_line() > second.get_end_line() {
            first.get_end_col()
        } else {
            second.get_end_col()
        };

        Span::new(start_line, start_col, end_line, end_col)
    }};
}

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
