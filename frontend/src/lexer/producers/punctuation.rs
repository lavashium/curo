use crate::lexer::*;
use common::*;
use language::*;

pub struct PunctuationProducer;

impl Factory<Option<Token>, Lexer<'_>, LexerContext<'_, '_>> for PunctuationProducer {
    fn run(lexer: &mut Lexer, _ctx: &mut LexerContext) -> Option<Token> {
        let ch = lexer.peek()?;

        let kind = match ch {
            ';' => token_punctuation!(Semicolon),
            '(' => token_punctuation!(OpenParen),
            ')' => token_punctuation!(CloseParen),
            '{' => token_punctuation!(OpenBrace),
            '}' => token_punctuation!(CloseBrace),
            ':' => token_punctuation!(Colon),
            ',' => token_punctuation!(Comma),
            _ => return None,
        };

        let start_pos = lexer.current_position();

        lexer.advance();

        let span = lexer.span_from(start_pos);
        let lexeme = ch.to_string();

        Some(Token::new(kind, lexeme, span))
    }
}
