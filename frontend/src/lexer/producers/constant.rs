use std::marker::PhantomData;

use crate::lexer::*;
use common::*;
use language::*;

pub struct ConstantProducer<'scp, 'ctx> {
    _driver: PhantomData<LexerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for ConstantProducer<'scp, 'ctx> {
    type Context = LexerContext<'scp, 'ctx>;
}

impl Factory<Option<Token>, Lexer<'_>> for ConstantProducer<'_, '_> {
    fn run(lexer: &mut Lexer, ctx: &mut LexerContext) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.get_pointer();

        let diagnostics = ctx.ctx.diagnostics_mut();

        let ch = lexer.peek()?;
        if !ch.is_ascii_digit() {
            return None;
        }

        while let Some(c) = lexer.peek() {
            if c.is_ascii_digit() {
                lexer.advance();
            } else {
                break;
            }
        }

        if let Some(c) = lexer.peek() {
            if c.is_ascii_alphabetic() || c == '_' {
                while let Some(c) = lexer.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        lexer.advance();
                    } else {
                        break;
                    }
                }

                let end_ptr = lexer.get_pointer();
                let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();
                let span = lexer.span_from(start_pos);

                let invalid_token = Token::new(TokenKind::Unknown(lexeme.clone()), lexeme.clone(), span);

                diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::Lexical(LexicalError::UnknownToken { token: invalid_token.clone() }),
                ));

                return Some(invalid_token);
            }
        }

        let end_ptr = lexer.get_pointer();
        let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();
        let span = lexer.span_from(start_pos);

        let kind = TokenKind::Constant(lexeme.clone());
        Some(Token::new(kind, lexeme, span))
    }
}
