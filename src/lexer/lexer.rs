use crate::token::*;
use crate::lexer::producer::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    source_code: &'a str,
    pointer: usize,
    line: usize,
    column: usize
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str) -> Self {
        assert!(source_code.is_ascii());
        Lexer {
            source_code,
            pointer: 0,
            line: 0,
            column: 0
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source_code
            .as_bytes()
            .get(self.pointer)
            .map(|&b| { b as char })
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

    pub fn parse(&mut self) -> TokenStream {
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

                tokens.push(Token::new(TokenKind::Unknown(ch.to_string()), ch.to_string(), span));
            }
        }

        TokenStream::new(tokens)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.parse();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind()).collect();

        assert!(kinds.is_empty());
    }

    #[test]
    fn test_single_char_tokens() {
        let mut lexer = Lexer::new("(){;}");
        let tokens = lexer.parse();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                &token_punctuation!(OpenParen),
                &token_punctuation!(CloseParen),
                &token_punctuation!(OpenBrace),
                &token_punctuation!(Semicolon),
                &token_punctuation!(CloseBrace),
            ]
        );
    }

    #[test]
    fn test_identifier_and_keyword() {
        let mut lexer = Lexer::new("int x");
        let tokens = lexer.parse();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                &token_keyword!(Int),
                &token_identifier!("x"),
            ]
        );
    }

    #[test]
    fn test_number_and_whitespace() {
        let mut lexer = Lexer::new("  42\t\n");
        let tokens = lexer.parse();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                &token_constant!(42),
            ]
        );
    }

    #[test]
    fn test_unknown_token() {
        let mut lexer = Lexer::new("@");
        let tokens = lexer.parse();
        let kind = tokens.iter().next().unwrap().kind();

        assert!(matches!(kind, TokenKind::Unknown(s) if s == "@"));
    }

    #[test]
    fn test_multiline() {
        let mut lexer = Lexer::new("int\nmain");
        let tokens = lexer.parse();
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                &token_keyword!(Int),
                &token_identifier!("main"),
            ]
        );
    }
}