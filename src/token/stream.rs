use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        TokenStream { tokens }
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn consume_expected(&mut self, expected: TokenKind) -> Option<Token> {
        match self.peek() {
            Some(actual) if actual.kind == expected => self.consume(),
            _ => None,
        }
    }

    pub fn any_tokens(&self) -> bool {
        !self.tokens.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Token> {
        self.tokens.iter().rev()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume()
    }
}