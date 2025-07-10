use super::*;
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

pub trait ExpressionParser {
    fn parse_expression(&mut self) -> ParseResult<AstExpression>;
    fn parse_binary_expression(&mut self, min_prec: u8) -> ParseResult<AstExpression>;
    fn parse_primary_expression(&mut self) -> ParseResult<AstExpression>;
}

impl<'a> ExpressionParser for ParserRules<'a> {
    fn parse_expression(&mut self) -> ParseResult<AstExpression> {
        self.parse_binary_expression(0)
    }

    fn parse_primary_expression(&mut self) -> ParseResult<AstExpression> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Constant(_) => {
                let span = self.parser.source_tokens.peek()?.get_span();
                let constant = self.unwrap_constant()?;
                Some(AstExpression::Constant { constant, span })
            }

            TokenKind::Identifier(_) => {
                let span = self.parser.source_tokens.peek()?.get_span();
                let identifier = self.unwrap_identifier()?;
                Some(AstExpression::Var { identifier, span })
            }

            TokenKind::Operator(op @ OperatorKind::Tilde) |
            TokenKind::Operator(op @ OperatorKind::Exclamation) |
            TokenKind::Operator(op @ OperatorKind::Minus) => {
                let span = self.parser.source_tokens.peek()?.get_span();
                let operator = op.to_unary()?.clone();
                self.parser.source_tokens.consume();
                let operand = self.parse_binary_expression(100)?;
                Some(AstExpression::Unary {
                    operator,
                    operand: Box::new(operand),
                    span,
                })
            }

            TokenKind::Punctuation(PunctuationKind::OpenParen) => {
                self.expect(token_punctuation!(OpenParen))?;
                let expr = self.parse_expression()?;
                self.expect(token_punctuation!(CloseParen))?;
                Some(expr)
            }

            _ => {
                let token = self.parser.source_tokens.peek()?;
                self.diagnostics.push(
                    Diagnostic::error(
                        token.get_span(),
                        DiagnosticKind::UnknownToken(token.clone()),
                    )
                    .with(Diagnostic::note(token.get_span(), "expected an expression here")),
                );
                None
            }
        }
    }

    fn parse_binary_expression(&mut self, min_prec: u8) -> ParseResult<AstExpression> {
        let start_span = self.parser.source_tokens.peek()?.get_span();
        let mut lhs = self.parse_primary_expression()?;

        loop {
            let next_token = match self.parser.source_tokens.peek() {
                Some(tok) => tok,
                None => break,
            };

            if let TokenKind::Operator(OperatorKind::Question) = next_token.kind() {
                if 3 < min_prec {
                    break;
                }

                self.parser.source_tokens.consume()?;
                let then_expr = self.parse_expression()?;
                self.expect(token_punctuation!(Colon))?;
                let else_expr = self.parse_binary_expression(3)?;

                let end_span = self.parser.source_tokens.peek()?.get_span();
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
            if prec < min_prec {
                break;
            }

            self.parser.source_tokens.consume()?;

            let next_min_prec = if op_kind == OperatorKind::Equal {
                prec // right-associative
            } else {
                prec + 1
            };

            let rhs = self.parse_binary_expression(next_min_prec)?;

            let end_span = self.parser.source_tokens.peek()?.get_span();
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