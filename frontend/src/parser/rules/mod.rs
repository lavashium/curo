mod expression;
mod program;
mod statement;
mod block_item;
mod declaration;
mod function_declaration;
mod variable_declaration;
mod block;
mod for_init;
mod param_list;
mod arg_list;

use crate::*;
use accessors::accessors;
use constructors::constructors;
use common::*;
use language::*;

macro_rules! push_eof_error {
    ($ctx:expr) => {
        $ctx.ctx.diagnostics.push(Diagnostic::error(
            Span::default(),
            DiagnosticKind::UnexpectedEof,
        ));
    };
}

macro_rules! error_expect {
    ($self:expr, $ctx:expr, $kind:expr) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) if token.kind() == &$kind => Some(token),
            Some(found) => {
                $ctx.ctx.diagnostics.push(Diagnostic::error(
                    found.get_span(),
                    DiagnosticKind::new_expected_token($kind, found.clone()),
                ));
                None
            }
            None => {
                push_eof_error!($ctx);
                None
            }
        }
    }};
}

macro_rules! error_consume_unwrap {
    ($self:expr, $ctx:expr, $kind:ident) => {{
        match $self.parser.source_tokens.consume() {
            Some(token) => match token.kind() {
                TokenKind::$kind(inner) => Some(inner.to_owned()),
                _ => {
                    let expected_kind = GenericKind::$kind;
                    $ctx.ctx.diagnostics.push(Diagnostic::error(
                        token.get_span(),
                        DiagnosticKind::new_unexpected_generic(token.clone(), vec![expected_kind]),
                    ));
                    None
                }
            },
            None => {
                push_eof_error!($ctx);
                None
            }
        }
    }};
}


#[constructors]
#[accessors]
pub struct ParserRules<'scp> {
    parser: &'scp mut Parser<'scp>,
}

impl<'a> ParserRules<'a> {
    pub fn expect(&mut self, ctx: &mut ParserContext, kind: TokenKind) -> Option<Token> {
        error_expect!(self, ctx, kind)
    }

    #[allow(dead_code)]
    pub fn unwrap_identifier(&mut self, ctx: &mut ParserContext) -> Option<String> {
        error_consume_unwrap!(self, ctx, Identifier)
    }

    #[allow(dead_code)]
    pub fn unwrap_constant(&mut self, ctx: &mut ParserContext) -> Option<String> {
        error_consume_unwrap!(self, ctx, Constant)
    }

    #[allow(dead_code)]
    pub fn unwrap_unknown(&mut self, ctx: &mut ParserContext) -> Option<String> {
        error_consume_unwrap!(self, ctx, Unknown)
    }

    #[allow(dead_code)]
    pub fn unwrap_operator(&mut self, ctx: &mut ParserContext) -> Option<OperatorKind> {
        error_consume_unwrap!(self, ctx, Operator)
    }

    #[allow(dead_code)]
    pub fn unwrap_keyword(&mut self, ctx: &mut ParserContext) -> Option<KeywordKind> {
        error_consume_unwrap!(self, ctx, Keyword)
    }

    #[allow(dead_code)]
    pub fn unwrap_punctuation(&mut self, ctx: &mut ParserContext) -> Option<PunctuationKind> {
        error_consume_unwrap!(self, ctx, Punctuation)
    }
}