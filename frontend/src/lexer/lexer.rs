use crate::lexer::producer::*;
use common::error::manager::DiagnosticsManager;
use common::*;
use language::token::*;

#[derive(Debug)]
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

    pub fn peek_slice(&mut self, span: (usize, usize)) -> Option<&str> {
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

    pub fn pointer(&self) -> usize {
        self.pointer
    }

    pub fn span_from(&self, start: (usize, usize)) -> Span {
        Span {
            start_line: start.0,
            start_col: start.1,
            end_line: self.line,
            end_col: self.column,
        }
    }

    pub fn parse(&mut self, diagnostics: &mut DiagnosticsManager) -> TokenStream {
        let mut tokens = Vec::new();

        while self.peek().is_some() {
            if let Some(token) = PRODUCERS::try_all(self) {
                if !matches!(token.kind, TokenKind::Unknown(ref s) if s == "Whitespace") {
                    tokens.push(token)
                }
            } else {
                let ch = self.advance().unwrap();
                let start_pos = self.current_position();

                let span = Span {
                    start_line: start_pos.0,
                    start_col: start_pos.1,
                    end_line: self.line,
                    end_col: self.column,
                };

                let token = Token::new(TokenKind::Unknown(ch.to_string()), ch.to_string(), span);

                diagnostics.push(errkind_error!(span, error_unknown_token!(token)));
            }
        }

        let eof_span = Span {
            start_line: self.line,
            start_col: self.column,
            end_line: self.line,
            end_col: self.column,
        };

        tokens.push(Token::new(TokenKind::EOF, String::new(), eof_span));

        TokenStream::new(tokens)
    }
}
