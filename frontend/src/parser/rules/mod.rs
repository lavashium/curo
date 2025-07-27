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
        $ctx.ctx.diagnostics.push(
            Diagnostic::error(
                Span::default(),
                DiagnosticKind::Lexical(LexicalError::UnexpectedEof),
            )
        );
    };
}

macro_rules! error_expect {
    ($self:expr, $ctx:expr, $kind:expr) => {{
        match $self.parser.source_tokens.consume() {
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
    ($self:expr, $ctx:expr, $variant:ident, $expected_kind:expr) => {{
        match $self.parser.source_tokens.consume() {
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
        error_consume_unwrap!(
            self,
            ctx,
            Identifier,
            TokenKind::Identifier(String::new())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_constant(&mut self, ctx: &mut ParserContext) -> Option<String> {
        error_consume_unwrap!(
            self,
            ctx,
            Constant,
            TokenKind::Constant(String::new())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_operator(&mut self, ctx: &mut ParserContext, expected_op: OperatorKind) -> Option<OperatorKind> {
        error_consume_unwrap!(
            self,
            ctx,
            Operator,
            TokenKind::Operator(expected_op.clone())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_keyword(&mut self, ctx: &mut ParserContext, expected_kw: KeywordKind) -> Option<KeywordKind> {
        error_consume_unwrap!(
            self,
            ctx,
            Keyword,
            TokenKind::Keyword(expected_kw.clone())
        )
    }

    #[allow(dead_code)]
    pub fn unwrap_punctuation(&mut self, ctx: &mut ParserContext, expected_punct: PunctuationKind) -> Option<PunctuationKind> {
        error_consume_unwrap!(
            self,
            ctx,
            Punctuation,
            TokenKind::Punctuation(expected_punct.clone())
        )
    }
}