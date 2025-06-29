use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::{DiagnosticsManager, errkind_error, error_custom};
use language::*;

pub struct CommentProducer;

impl TokenProducer for CommentProducer {
    fn try_match(lexer: &mut Lexer, diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let start_pointer = lexer.pointer();

        let next_two = lexer.peek_slice((start_pointer, start_pointer + 2))?;

        if next_two == "//" {
            lexer.advance();
            lexer.advance();
            while let Some(ch) = lexer.peek() {
                if ch == '\n' {
                    break;
                }
                lexer.advance();
            }
            return Some(Token::new(
                TokenKind::Irrelevant,
                "".to_string(),
                Span::default(),
            ));
        }

        if next_two == "/*" {
            lexer.advance();
            lexer.advance();

            while let Some(_) = lexer.peek() {
                let pos = lexer.pointer();
                if let Some(slice) = lexer.peek_slice((pos, pos + 2)) {
                    if slice == "*/" {
                        lexer.advance();
                        lexer.advance();
                        return Some(Token::new(
                            TokenKind::Irrelevant,
                            "".to_string(),
                            Span::default(),
                        ));
                    }
                } else {
                    break;
                }
                lexer.advance();
            }
            diagnostics.push(errkind_error!(
                Span::default(),
                error_custom!("Unterminated multi-line comment")
            ));
            return Some(Token::new(
                TokenKind::Irrelevant,
                "".to_string(),
                Span::default(),
            ));
        }

        None
    }
}
