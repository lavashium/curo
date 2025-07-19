use crate::lexer::*;
use common::*;
use language::*;

pub struct WhitespaceProducer;

impl Factory<Option<Token>, Lexer<'_>, LexerContext<'_, '_>> for WhitespaceProducer {
    fn run(lexer: &mut Lexer, _ctx: &mut LexerContext) -> Option<Token> {
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
