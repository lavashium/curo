use crate::*;
use super::*;
use common::*;
use language::*;

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

impl<'a> ParserRules<'a> {
    pub fn parse_expression(&mut self, ctx: &mut ParserContext) -> Option<AstExpression> {
        <Self as Factory<Option<AstExpression>, Self, ParserContext>>::run(self, ctx)
    }

    pub fn parse_binary_expression(&mut self, ctx: &mut ParserContext, min_prec: u8) -> Option<AstExpression> {
        let old_prec = ctx.min_prec;
        ctx.min_prec = min_prec;
        let result = ParseBinaryExpression::run(self, ctx);
        ctx.min_prec = old_prec;
        result
    }

    pub fn parse_primary_expression(&mut self, ctx: &mut ParserContext) -> Option<AstExpression> {
        ParsePrimaryExpression::run(self, ctx)
    }
}

impl<'a> Factory<Option<AstExpression>, ParserRules<'a>, ParserContext<'_, '_>> for ParserRules<'a> {
    fn run(rules: &mut ParserRules<'a>, ctx: &mut ParserContext) -> Option<AstExpression> {
        rules.parse_binary_expression(ctx, 0)
    }
}

pub struct ParseBinaryExpression;

impl<'a> Factory<Option<AstExpression>, ParserRules<'a>, ParserContext<'_, '_>> for ParseBinaryExpression {
    fn run(rules: &mut ParserRules<'a>, ctx: &mut ParserContext) -> Option<AstExpression> {
        let start_span = rules.parser.source_tokens.peek()?.get_span();
        let mut lhs = rules.parse_primary_expression(ctx)?;

        loop {
            let next_token = match rules.parser.source_tokens.peek() {
                Some(tok) => tok,
                None => break,
            };

            if let TokenKind::Operator(OperatorKind::Question) = next_token.kind() {
                if 3 < ctx.min_prec {
                    break;
                }

                rules.parser.source_tokens.consume()?;
                let then_expr = rules.parse_expression(ctx)?;
                rules.expect(ctx, token_punctuation!(Colon))?;
                let else_expr = rules.parse_binary_expression(ctx, 3)?;

                let end_span = rules.parser.source_tokens.peek()?.get_span();
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

            rules.parser.source_tokens.consume()?;

            let next_min_prec = if op_kind == OperatorKind::Equal {
                prec
            } else {
                prec + 1
            };

            let rhs = rules.parse_binary_expression(ctx, next_min_prec)?;

            let end_span = rules.parser.source_tokens.peek()?.get_span();
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

pub struct ParsePrimaryExpression;

impl<'a> Factory<Option<AstExpression>, ParserRules<'a>, ParserContext<'_, '_>> for ParsePrimaryExpression {
    fn run(rules: &mut ParserRules<'a>, ctx: &mut ParserContext) -> Option<AstExpression> {
        match rules.parser.source_tokens.peek()?.kind() {
            TokenKind::Constant(_) => {
                let span = rules.parser.source_tokens.peek()?.get_span();
                let constant = rules.unwrap_constant(ctx)?;
                Some(AstExpression::Constant { constant, span })
            }

            TokenKind::Identifier(_) => {
                let span = rules.parser.source_tokens.peek()?.get_span();
                let name = rules.unwrap_identifier(ctx)?;

                if rules.parser.source_tokens.peek()?.kind() == &token_punctuation!(OpenParen) {
                    rules.parser.source_tokens.consume_expected(&token_punctuation!(OpenParen))?;
                    let args = if rules.parser.source_tokens.peek()?.kind() != &token_punctuation!(CloseParen) {
                        rules.parse_argument_list(ctx)?
                    } else {
                        Vec::new()
                    };
                    rules.expect(ctx, token_punctuation!(CloseParen))?;
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
                let span = rules.parser.source_tokens.peek()?.get_span();
                let operator = op.to_unary()?.clone();
                rules.parser.source_tokens.consume()?;
                let operand = rules.parse_binary_expression(ctx, 100)?;
                Some(AstExpression::Unary {
                    operator,
                    operand: Box::new(operand),
                    span,
                })
            }

            TokenKind::Punctuation(PunctuationKind::OpenParen) => {
                rules.expect(ctx, token_punctuation!(OpenParen))?;
                let expr = rules.parse_expression(ctx)?;
                rules.expect(ctx, token_punctuation!(CloseParen))?;
                Some(expr)
            }

            _ => {
                let token = rules.parser.source_tokens.peek()?;
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
