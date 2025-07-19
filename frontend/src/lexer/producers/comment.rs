use crate::lexer::*;
use common::*;
use language::*;

pub struct CommentProducer;

impl Factory<Option<Token>, Lexer<'_>, LexerContext<'_, '_>> for CommentProducer {
    fn run(lexer: &mut Lexer, ctx: &mut LexerContext) -> Option<Token> {
        let start_pointer = lexer.get_pointer();

        let diagnostics = ctx.ctx.diagnostics_mut();

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
                let pos = lexer.get_pointer();
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
            diagnostics.push(Diagnostic::error(
                Span::default(),
                DiagnosticKind::Custom("Unterminated multi-line comment".to_string()),
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
