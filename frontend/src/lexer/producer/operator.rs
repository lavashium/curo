use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::*;
use language::*;

pub struct OperatorProducer;

impl TokenProducer for OperatorProducer {
    fn try_match(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.pointer();

        if let Some(slice) = lexer.peek_slice((start_ptr, start_ptr + 2)) {
            match slice {
                "--" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(errkind_error!(
                        span,
                        error_custom!("Decrement operator is NOT supported")
                    ));
                    return None;
                }
                "++" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(errkind_error!(
                        span,
                        error_custom!("Increment operator is NOT supported")
                    ));
                    return None;
                }
                _ => {}
            }
        }

        let ch = lexer.peek()?;
        let kind = match ch {
            '-' => token_operator!(Negation),
            '~' => token_operator!(Complement),
            _ => return None,
        };

        lexer.advance();

        let span = lexer.span_from(start_pos);
        let lexeme = ch.to_string();

        Some(Token::new(kind, lexeme, span))
    }
}
