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

use std::marker::PhantomData;

use crate::*;
use common::*;
use language::*;

#[macro_export]
macro_rules! try_apply {
    ($factory:ty, $product:ty, $input:expr, $ctx:expr) => {{
        let output: Option<$product> = <$factory as Factory<Option<$product>, _>>::run($input, $ctx);
        output?
    }};
}

macro_rules! push_eof_error {
    ($ctx:expr) => {
        $ctx.ctx.diagnostics.push(
            Diagnostic::error(
                Span::default(),
                DiagnosticKind::Lexical(LexicalError::UnexpectedEof),
            )
        );
    };
}

macro_rules! error_expect {
    ($stream:expr, $ctx:expr, $kind:expr) => {{
        match $stream.consume() {
            Some(tok) if tok.kind() == &$kind => Some(tok),
            Some(found) => {
                $ctx.ctx.diagnostics.push(
                    Diagnostic::error(
                        found.get_span(),
                        DiagnosticKind::Syntax(SyntaxError::ExpectedToken {
                            expected: $kind.clone(),
                            found: found.clone(),
                        })
                    )
                );
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
    ($stream:expr, $ctx:expr, $variant:ident, $expected_kind:expr) => {{
        match $stream.consume() {
            Some(tok) => {
                if let TokenKind::$variant(inner) = tok.kind() {
                    Some(inner.clone())
                } else {
                    $ctx.ctx.diagnostics.push(
                        Diagnostic::error(
                            tok.get_span(),
                            DiagnosticKind::Syntax(SyntaxError::ExpectedToken {
                                expected: $expected_kind.clone(),
                                found: tok.clone(),
                            })
                        )
                    );
                    None
                }
            }
            None => {
                push_eof_error!($ctx);
                None
            }
        }
    }};
}

pub struct ParserRules<'scp, 'ctx> {
    _driver: PhantomData<ParserContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for ParserRules<'scp, 'ctx> {
    type Context = ParserContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> ParserRules<'scp, 'ctx> {
    pub fn expect(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>, kind: TokenKind) -> Option<Token> {
        error_expect!(stream, ctx, kind)
    }

    #[allow(dead_code)]
    pub fn unwrap_identifier(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<String> {
        error_consume_unwrap!(
            stream,
            ctx,
            Identifier,
            TokenKind::Identifier(String::new())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_constant(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<String> {
        error_consume_unwrap!(
            stream,
            ctx,
            Constant,
            TokenKind::Constant(String::new())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_operator(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>, expected_op: OperatorKind) -> Option<OperatorKind> {
        error_consume_unwrap!(
            stream,
            ctx,
            Operator,
            TokenKind::Operator(expected_op.clone())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_keyword(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>, expected_kw: KeywordKind) -> Option<KeywordKind> {
        error_consume_unwrap!(
            stream,
            ctx,
            Keyword,
            TokenKind::Keyword(expected_kw.clone())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_punctuation(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>, expected_punct: PunctuationKind) -> Option<PunctuationKind> {
        error_consume_unwrap!(
            stream,
            ctx,
            Punctuation,
            TokenKind::Punctuation(expected_punct.clone())
        )
    }
}