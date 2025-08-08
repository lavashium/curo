use std::marker::PhantomData;

use crate::lexer::*;
use common::*;
use language::*;

pub struct WhitespaceProducer<'scp, 'ctx> {
    _driver: PhantomData<LexerContext<'scp, 'ctx>>,
}

impl<'scp, 'ctx> Driver for WhitespaceProducer<'scp, 'ctx> {
    type Context = LexerContext<'scp, 'ctx>;
}

impl Factory<Option<Token>, Lexer<'_>> for WhitespaceProducer<'_, '_> {
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
