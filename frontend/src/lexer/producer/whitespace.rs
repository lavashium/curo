use crate::lexer::Lexer;
use crate::lexer::producer::TokenProducer;
use common::DiagnosticsManager;
use language::*;

pub struct WhitespaceProducer;

impl TokenProducer for WhitespaceProducer {
    fn try_match(lexer: &mut Lexer, _diagnostics: &mut DiagnosticsManager) -> Option<Token> {
        let ch = lexer.peek()?;
        if !ch.is_whitespace() {
            return None;
        }

        while let Some(c) = lexer.peek() {
            if c.is_whitespace() {
                lexer.advance();
            } else {
                break;
            }
        }

        Some(Token::new(
            TokenKind::Irrelevant,
            "".to_string(),
            Span::default(),
        ))
    }
}