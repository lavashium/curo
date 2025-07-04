use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::*;
use language::*;

pub struct OperatorProducer;

impl TokenProducer for OperatorProducer {
    fn try_match(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let start_pos = lexer.current_position();
        let start_ptr = lexer.get_pointer();

        if let Some(slice) = lexer.peek_slice((start_ptr, start_ptr + 2)) {
            match slice {
                "--" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(Diagnostic::error(
                        span,
                        DiagnosticKind::Custom("Decrement operator is NOT supported".to_string())
                    ));
                    return None;
                }
                "++" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(Diagnostic::error(
                        span,
                        DiagnosticKind::Custom("Increment operator is NOT supported".to_string())
                    ));
                    return None;
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
            '/' => token_operator!(ForwardSlash),
            '%' => token_operator!(PercentSign),
            _ => return None,
        };

        lexer.advance();

        let span = lexer.span_from(start_pos);
        let lexeme = ch.to_string();

        Some(Token::new(kind, lexeme, span))
    }
}
