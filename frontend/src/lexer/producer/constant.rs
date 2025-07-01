use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::*;
use language::*;

pub struct ConstantProducer;

impl TokenProducer for ConstantProducer {
    fn try_match(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.pointer();

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

                let end_ptr = lexer.pointer();
                let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();
                let span = lexer.span_from(start_pos);

                let invalid_token =
                    Token::new(TokenKind::Unknown(lexeme.clone()), lexeme.clone(), span);
                let diag = errkind_error!(span, error_unknown_token!(invalid_token.clone()));
                diagnostics.push(diag);

                return Some(invalid_token);
            }
        }

        let end_ptr = lexer.pointer();
        let lexeme = lexer.peek_slice((start_ptr, end_ptr))?.to_string();
        let span = lexer.span_from(start_pos);

        let kind = TokenKind::Constant(lexeme.clone());
        Some(Token::new(kind, lexeme, span))
    }
}
