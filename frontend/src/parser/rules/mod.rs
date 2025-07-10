mod expression;
mod function;
mod program;
mod statement;
mod block_item;
mod declaration;
mod block;

pub use expression::*;
pub use function::*;
pub use program::*;
pub use statement::*;
pub use block_item::*;
pub use declaration::*;
pub use block::*;

use crate::Parser;
use accessors::accessors;
use common::*;
use constructors::constructors;
use language::*;

macro_rules! push_eof_error {
    ($diagnostics:expr) => {
        $diagnostics.push(Diagnostic::error(
            Span::default(),
            DiagnosticKind::UnexpectedEof,
        ));
    };
}

macro_rules! error_expect {
    ($self:expr, $kind:expr) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) if token.kind() == &$kind => Some(token),
            Some(found) => {
                $self.diagnostics.push(Diagnostic::error(
                    found.get_span(),
                    DiagnosticKind::new_expected_token($kind, found.clone()),
                ));
                None
            }
            None => {
                push_eof_error!($self.diagnostics);
                None
            }
        }
    }};
}

macro_rules! error_consume_unwrap {
    ($self:expr, $kind:ident) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) => match token.kind() {
                TokenKind::$kind(inner) => Some(inner.to_owned()),
                _ => {
                    let expected_kind = GenericKind::$kind;
                    $self.diagnostics.push(Diagnostic::error(
                        token.get_span(),
                        DiagnosticKind::new_unexpected_generic(token.clone(), vec![expected_kind]),
                    ));
                    None
                }
            },
            None => {
                push_eof_error!($self.diagnostics);
                None
            }
        }
    }};
}

pub type ParseResult<N> = Option<N>;

#[constructors]
#[accessors]
pub struct ParserRules<'a> {
    parser: &'a mut Parser,
    diagnostics: &'a mut DiagnosticsManager,
}

impl<'a> ParserRules<'a> {
    pub fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        error_expect!(self, kind)
    }

    #[allow(dead_code)]
    pub fn unwrap_identifier(&mut self) -> Option<String> {
        error_consume_unwrap!(self, Identifier)
    }

    #[allow(dead_code)]
    pub fn unwrap_constant(&mut self) -> Option<String> {
        error_consume_unwrap!(self, Constant)
    }

    #[allow(dead_code)]
    pub fn unwrap_unknown(&mut self) -> Option<String> {
        error_consume_unwrap!(self, Unknown)
    }

    #[allow(dead_code)]
    pub fn unwrap_operator(&mut self) -> Option<OperatorKind> {
        error_consume_unwrap!(self, Operator)
    }

    #[allow(dead_code)]
    pub fn unwrap_keyword(&mut self) -> Option<KeywordKind> {
        error_consume_unwrap!(self, Keyword)
    }

    #[allow(dead_code)]
    pub fn unwrap_punctuation(&mut self) -> Option<PunctuationKind> {
        error_consume_unwrap!(self, Punctuation)
    }
}
