use language::*;
use super::*;

pub trait ExpressionParser<'a> {
    fn parse_expression(&mut self) -> ParseResult<AstExpression>;
}

impl<'a> ExpressionParser<'a> for ParserRules<'a> {
    fn parse_expression(&mut self) -> ParseResult<AstExpression> {
        match self.parser.source_tokens.peek()?.kind() {
            TokenKind::Constant(_) => {
                let constant = self.unwrap_constant()?;
                Some(AstExpression::Constant { constant })
            }

            TokenKind::Operator(OperatorKind::Complement)
            | TokenKind::Operator(OperatorKind::Negation) => {
                let operator = match self.parser.source_tokens.consume()?.kind() {
                    TokenKind::Operator(OperatorKind::Complement) => UnaryKind::Complement,
                    TokenKind::Operator(OperatorKind::Negation) => UnaryKind::Negation,
                    _ => {
                        let token = self.parser.source_tokens.peek()?;
                        self.diagnostics.push(
                            errkind_error!(token.span, error_unknown_token!(token.clone()))
                                .with(errkind_note!(token.span, "expected a unary operator here")),
                        );
                        return None;
                    }
                };

                let operand = self.parse_expression()?;
                Some(AstExpression::Unary {
                    operator,
                    operand: Box::new(operand),
                })
            }

            TokenKind::Punctuation(PunctuationKind::OpenParen) => {
                self.expect(token_punctuation!(OpenParen))?;
                let inner = self.parse_expression()?;
                self.expect(token_punctuation!(CloseParen))?;
                Some(inner)
            }

            _ => {
                let token = self.parser.source_tokens.peek()?;
                self.diagnostics.push(
                    errkind_error!(token.span, error_unknown_token!(token.clone()))
                        .with(errkind_note!(token.span, "expected an expression here")),
                );
                None
            }
        }
    }
}
