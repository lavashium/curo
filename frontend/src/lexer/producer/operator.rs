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
                        DiagnosticKind::Custom("Decrement operator is NOT supported".to_string()),
                    ));
                    return None;
                }
                "++" => {
                    lexer.advance();
                    lexer.advance();

                    let span = lexer.span_from(start_pos);
                    diagnostics.push(Diagnostic::error(
                        span,
                        DiagnosticKind::Custom("Increment operator is NOT supported".to_string()),
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
            _ => return None,
        };

        let lexeme = ch.to_string();
        lexer.advance();

        let span = lexer.span_from(start_pos);

        Some(Token::new(kind, lexeme, span))
    }
}