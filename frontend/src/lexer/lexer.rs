use super::producers::*;
use super::*;
use accessors::accessors;
use common::*;
use language::token::*;
use std::rc::Rc;
use zawarudo::zawarudo;

#[accessors] 
pub struct Lexer<'a> {
    source_code: &'a str,
    pointer: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        assert!(source_code.is_ascii());
        Lexer {
            source_code,
            pointer: 0,
            line: 0,
            column: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source_code
            .as_bytes()
            .get(self.pointer)
            .map(|&b| b as char)
    }

    pub fn peek_slice(&self, span: (usize, usize)) -> Option<&str> {
        let (start, end) = span;
        self.source_code.get(start..end)
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pointer += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    pub fn current_position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn span_from(&self, start: (usize, usize)) -> Span {
        Span::new(start.0, start.1, self.line, self.column)
    }

    #[zawarudo(label = "Lexer")]
    pub fn parse(&mut self, ctx: &mut LexerContext) -> TokenStream {
        let mut tokens = Vec::new();

        while self.peek().is_some() {
            if let Some(token) = PRODUCERS::run(self, ctx) {
                if !matches!(token.kind(), TokenKind::Irrelevant) {
                    tokens.push(token);
                }
            } else {
                let ch = self.advance().unwrap();

                let span = Span::new(self.line, self.column, self.line, self.column);

                let token = Token::new(TokenKind::Unknown(ch.to_string()), ch.to_string(), span);

                ctx.ctx.diagnostics.push(Diagnostic::error(
                    span,
                    DiagnosticKind::Lexical(LexicalError::UnknownToken { token }),
                ));
            }
        }

        let eof_span = Span::new(self.line, self.column, self.line, self.column);

        tokens.push(Token::new(TokenKind::EOF, String::new(), eof_span));

        TokenStream::new(Rc::from(tokens))
    }
}
