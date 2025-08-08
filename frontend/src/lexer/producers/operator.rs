use std::marker::PhantomData;

use crate::lexer::*;
use common::*;
use language::*;

pub struct OperatorProducer<'scp, 'ctx> {
    _driver: PhantomData<LexerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for OperatorProducer<'scp, 'ctx> {
    type Context = LexerContext<'scp, 'ctx>;
}

impl Factory<Option<Token>, Lexer<'_>> for OperatorProducer<'_, '_> {
    fn run(lexer: &mut Lexer, ctx: &mut LexerContext) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.get_pointer();

        let diagnostics = ctx.ctx.diagnostics_mut();

        if let Some(slice) = lexer.peek_slice((start_ptr, start_ptr + 2)) {
            match slice {
                "--" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(Diagnostic::error(
                        span,
                        DiagnosticKind::Custom(CustomError::Message("Decrement operator is NOT supported".into())),
                    ));
                    return None;
                }
                "++" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(Diagnostic::error(
                        span,
                       DiagnosticKind::Custom(CustomError::Message("Increment operator is NOT supported".into())),
                    ));
                    return None;
                }
                "&&" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = "&&".to_string();
                    let kind = token_operator!(LogicalAnd);
                    return Some(Token::new(kind, lexeme, span));
                }
                "||" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = "||".to_string();
                    let kind = token_operator!(LogicalOr);
                    return Some(Token::new(kind, lexeme, span));
                }
                "==" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = "==".to_string();
                    let kind = token_operator!(EqualEqual);
                    return Some(Token::new(kind, lexeme, span));
                }
                "!=" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = "!=".to_string();
                    let kind = token_operator!(NotEqual);
                    return Some(Token::new(kind, lexeme, span));
                }
                "<=" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = "<=".to_string();
                    let kind = token_operator!(LessEqual);
                    return Some(Token::new(kind, lexeme, span));
                }
                ">=" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    let lexeme = ">=".to_string();
                    let kind = token_operator!(GreaterEqual);
                    return Some(Token::new(kind, lexeme, span));
                }
                _ => {}
            }
        }

        let ch = lexer.peek()?;
        let kind = match ch {
            '-' => token_operator!(Minus),
            '~' => token_operator!(Tilde),
            '+' => token_operator!(Plus),
            '*' => token_operator!(Asterisk),
            '/' => token_operator!(Slash),
            '%' => token_operator!(Percent),
            '!' => token_operator!(Exclamation),
            '<' => token_operator!(LessThan),
            '>' => token_operator!(GreaterThan),
            '=' => token_operator!(Equal),
            '?' => token_operator!(Question),
            _ => return None,
        };

        let lexeme = ch.to_string();
        lexer.advance();

        let span = lexer.span_from(start_pos);

        Some(Token::new(kind, lexeme, span))
    }
}