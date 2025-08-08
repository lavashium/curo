use crate::*;
use super::*;
use common::*;
use language::*;

impl<'scp, 'ctx> Factory<Option<AstExpression>, TokenStream> for ParserRules<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstExpression> {
        ParseBinaryExpression::run(stream, ctx)
    }
}

impl<'scp, 'ctx> ParserRules<'scp, 'ctx> {
    pub fn parse_expression(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstExpression> {
        Self::parse_binary_expression(stream, ctx, 0)
    }

    pub fn parse_binary_expression(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>, min_prec: u8) -> Option<AstExpression> {
        let old_prec = ctx.min_prec;
        ctx.min_prec = min_prec;
        let result = ParseBinaryExpression::run(stream, ctx);
        ctx.min_prec = old_prec;
        result
    }

    pub fn parse_primary_expression(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstExpression> {
        ParsePrimaryExpression::run(stream, ctx)
    }
}
pub struct ParseBinaryExpression<'scp, 'ctx> {
    _driver: PhantomData<ParserContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for ParseBinaryExpression<'scp, 'ctx> {
    type Context = ParserContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<Option<AstExpression>, TokenStream> for ParseBinaryExpression<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstExpression> {
        let start_span = stream.peek()?.get_span();
        let mut lhs = ParserRules::parse_primary_expression(stream, ctx)?;

        loop {
            let next_token = match stream.peek() {
                Some(tok) => tok,
                None => break,
            };

            if let TokenKind::Operator(OperatorKind::Question) = next_token.kind() {
                if 3 < ctx.min_prec {
                    break;
                }

                stream.consume()?;
                let then_expr = ParserRules::parse_expression(stream, ctx)?;
                ParserRules::expect(stream, ctx, token_punctuation!(Colon))?;
                let else_expr = ParserRules::parse_binary_expression(stream, ctx, 3)?;

                let end_span = stream.peek()?.get_span();
                let span = combine_spans!(start_span, end_span);

                lhs = AstExpression::Conditional {
                    condition: Box::new(lhs),
                    then_branch: Box::new(then_expr),
                    else_branch: Box::new(else_expr),
                    span,
                };

                continue;
            }

            let op_kind = match next_token.kind() {
                TokenKind::Operator(op) => op.clone(),
                _ => break,
            };

            let prec = get_precedence(&op_kind);
            if prec < ctx.min_prec {
                break;
            }

            stream.consume()?;

            let next_min_prec = if op_kind == OperatorKind::Equal {
                prec
            } else {
                prec + 1
            };

            let rhs = ParserRules::parse_binary_expression(stream, ctx, next_min_prec)?;

            let end_span = stream.peek()?.get_span();
            let span = combine_spans!(start_span, end_span);

            if op_kind == OperatorKind::Equal {
                lhs = AstExpression::Assignment {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    span,
                };
                continue;
            }

            let bin_op = match op_kind.to_binary() {
                Some(b) => b,
                None => break,
            };

            lhs = AstExpression::Binary {
                operator: bin_op,
                left: Box::new(lhs),
                right: Box::new(rhs),
                span,
            };
        }

        Some(lhs)
    }
}

pub struct ParsePrimaryExpression<'scp, 'ctx> {
    _driver: PhantomData<ParserContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for ParsePrimaryExpression<'scp, 'ctx> {
    type Context = ParserContext<'scp, 'ctx>;
}

impl<'scp, 'ctx> Factory<Option<AstExpression>, TokenStream> for ParsePrimaryExpression<'scp, 'ctx> {
    fn run(stream: &mut TokenStream, ctx: &mut ParserContext<'scp, 'ctx>) -> Option<AstExpression> {
        match stream.peek()?.kind() {
            TokenKind::Constant(_) => {
                let span = stream.peek()?.get_span();
                let constant = ParserRules::unwrap_constant(stream, ctx)?;
                Some(AstExpression::Constant { constant, span })
            }

            TokenKind::Identifier(_) => {
                let span = stream.peek()?.get_span();
                let name = ParserRules::unwrap_identifier(stream, ctx)?;

                if stream.peek()?.kind() == &token_punctuation!(OpenParen) {
                    stream.consume_expected(&token_punctuation!(OpenParen))?;
                    let args = if stream.peek()?.kind() != &token_punctuation!(CloseParen) {
                        try_apply!(ParserRules, _, stream, ctx)
                    } else {
                        Vec::new()
                    };
                    ParserRules::expect(stream, ctx, token_punctuation!(CloseParen))?;
                    Some(AstExpression::FunctionCall {
                        identifier: name,
                        args,
                        span,
                    })
                } else {
                    Some(AstExpression::Var { identifier: name, span })
                }
            }

            TokenKind::Operator(op @ OperatorKind::Tilde)
            | TokenKind::Operator(op @ OperatorKind::Exclamation)
            | TokenKind::Operator(op @ OperatorKind::Minus) => {
                let span = stream.peek()?.get_span();
                let operator = op.to_unary()?.clone();
                stream.consume()?;
                let operand = ParserRules::parse_binary_expression(stream, ctx, 100)?;
                Some(AstExpression::Unary {
                    operator,
                    operand: Box::new(operand),
                    span,
                })
            }

            TokenKind::Punctuation(PunctuationKind::OpenParen) => {
                ParserRules::expect(stream, ctx, token_punctuation!(OpenParen))?;
                let expr = ParserRules::parse_expression(stream, ctx)?;
                ParserRules::expect(stream, ctx, token_punctuation!(CloseParen))?;
                Some(expr)
            }

            _ => {
                let token = stream.peek()?;
                ctx.ctx.diagnostics.push(
                    Diagnostic::error(
                        token.get_span(),
                        DiagnosticKind::Lexical(LexicalError::UnknownToken { token: token.clone() }),
                    )
                    .with(
                        Diagnostic::note(
                            token.get_span(),
                            DiagnosticKind::Custom(CustomError::Message("expected an expression here".into()))
                        )
                    )
                );
                None
            }
        }
    }
}

macro_rules! precedence {
    ($operator:expr; $($op:ident => $val:expr),* $(,)?) => {
        match $operator {
            $(
                OperatorKind::$op => $val,
            )*
            _ => 0
        }
    }
}

fn get_precedence(operator: &OperatorKind) -> u8 {
    precedence!(operator;
        Asterisk      => 50,
        Slash         => 50,
        Percent       => 50,
        Plus          => 45,
        Minus         => 45,
        LessThan      => 35,
        LessEqual     => 35,
        GreaterThan   => 35,
        GreaterEqual  => 35,
        EqualEqual    => 30,
        NotEqual      => 30,
        LogicalAnd    => 10,
        LogicalOr     => 5,
        Question      => 3,
        Equal         => 1,
    )
}
